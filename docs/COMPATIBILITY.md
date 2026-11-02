# Compatibility list

Every Pocket PC download we could find on archive.org (1,443 files), imported into PocketPC3630 and run for 20 seconds in the web build, with three taps on the screen. **Runs** means it drew its own screen; it does not promise every feature works.

Also available as [compatibility.csv](compatibility.csv). Free programs that run can be played in one click on [the website](https://szupotseng.github.io/PocketPC3630/#free).

| | Result | Count |
|---|---|---:|
| ✅ | **Runs** — Starts and draws the game or program. | 93 |
| 🟡 | **Partly works** — Imports and starts, but the screen stays blank, draws nothing, or stops soon. | 156 |
| 🟥 | **Does not start** — Imports, but fails while starting (usually a missing system DLL or an API not emulated yet). | 390 |
| ⛔ | **Not importable** — No Pocket PC program inside the download (desktop tools, ROM updates, documents). | 384 |
| ⚪ | **Not tested** — The archive.org item has no package, or the download failed. | 420 |

## Most frequent missing APIs

Each program counted once. These are the next things to implement.

| API | Programs |
|---|---:|
| `COREDLL.dll!GetCaretPos` | 9 |
| `COREDLL.dll!GetOpenFileNameW` | 8 |
| `COREDLL.dll!Shell_NotifyIcon` | 7 |
| `COREDLL.dll!CreatePenIndirect` | 7 |
| `COREDLL.dll!ImageList_ReplaceIcon` | 7 |
| `WINSOCK.dll!WSAStartup` | 7 |
| `COREDLL.dll!RegisterClipboardFormatW` | 7 |
| `COREDLL.dll!GetVersionEx` | 7 |
| `COREDLL.dll!GetDateFormatW` | 7 |
| `COREDLL.dll!GetTimeFormatW` | 6 |
| `COREDLL.dll!LoadLibraryExW` | 6 |
| `IMGDECMP.DLL!#1` | 5 |
| `AYGSHELL.dll!#69` | 5 |
| `COREDLL.dll!SetScrollPos` | 5 |
| `DocList.dll!DocList_Create` | 5 |
| `COREDLL.dll!DestroyIcon` | 5 |
| `COREDLL.dll!GetTempPathW` | 5 |
| `COREDLL.dll!CreatePatternBrush` | 5 |
| `COREDLL.dll!SHGetShortcutTarget` | 5 |
| `COREDLL.dll!CeFindFirstDatabase` | 5 |

73 programs need `mfcce300.dll`, a library built into the device. 13 programs need `mfcce211.dll`, a library built into the device. 6 programs need `mfcce20.dll`, a library built into the device. 2 programs need `mfcce201.dll`, a library built into the device. 1 programs need `mfcce400.dll`, a library built into the device. The website adds `mfcce300.dll` from Microsoft's Pocket PC 2000 SDK in one click.

## ✅ Runs (93)

Starts and draws the game or program.

| Title | File | Result | Notes |
|---|---|---|---|
| [AlarMe](https://archive.org/details/tucows_32528_AlarMe) | `AlarMe_HPC.exe` | Runs | Missing: `COREDLL.dll!CreatePenIndirect`, `COREDLL.dll!CompareFileTime`, `COREDLL.dll!RegDeleteKeyW` |
| [AW RoadLingua](https://archive.org/details/tucows_272210_AW_RoadLingua) | `rl_ppc.zip` | Runs |  |
| [BasiCE](https://archive.org/details/tucows_32347_BasiCE) | `basice.zip` | Runs |  |
| [Bejeweled 2 for Pocket PCs](https://archive.org/details/bejeweled2-ppc) | `bejeweled2-ppc-v1-30-wm5.cab` | Runs |  |
| [Block Breaker Deluxe 2 HD (windows mobile 240x400)](https://archive.org/details/block-breaker-deluxe-2-hd) | `Block Breaker Deluxe 2 HD.cab` | Runs |  |
| [Blocks](https://archive.org/details/tucows_240055_Blocks_extracted) | `Blocks/install.PPC_ARM.CAB` | Runs |  |
| [Bomberman '08](https://archive.org/details/bomberman-08) | `Bomberman'08.cab` | Runs | Missing: `COREDLL.dll!CryptAcquireContextW`, `COREDLL.dll!ImmSetOpenStatus`, `COREDLL.dll!NLedGetDeviceInfo` |
| [Bookworm for Pocket PCs](https://archive.org/details/bookworm_wm5-6) | `bookworm_wm5-6.cab` | Runs | Missing: `COREDLL.dll!GetTimeFormatW` |
| [Bubble Shooter](https://archive.org/details/tucows_229202_Bubble_Shooter) | `bubshootce.exe` | Runs |  |
| [Bubble Thriller](https://archive.org/details/tucows_230188_Bubble_Thriller) | `thrillce.exe` | Runs |  |
| [Calc98](https://archive.org/details/tucows_212355_Calc98) | `c9853a.exe` | Runs | Missing: `COREDLL.dll!SetScrollRange`, `COREDLL.dll!SetScrollPos` |
| [Calc98 5.3 WinCE (Extracted)](https://archive.org/details/calc-9853-wince) | `CALC98S.CAB` | Runs | Missing: `COREDLL.dll!SetScrollRange`, `COREDLL.dll!SetScrollPos` |
| [Calc98 5.3 WinCE (Extracted)](https://archive.org/details/calc-9853-wince) | `CALC98A.CAB` | Runs | Missing: `COREDLL.dll!SetScrollRange`, `COREDLL.dll!SetScrollPos` |
| [Calendar Search](https://archive.org/details/tucows_32395_Calendar_Search) | `CalendarSearchMIPS-PR2.exe` | Runs | Missing: `COREDLL.dll!PegOpenDatabase`, `COREDLL.dll!PegOidGetInfo` |
| [CE-Star Suite](https://archive.org/details/tucows_153762_CE-Star_Suite) | `ver2PkPCARM.zip` | Runs | Missing: `COREDLL.dll!Shell_NotifyIcon` |
| [CE5250](https://archive.org/details/tucows_232364_CE5250) | `CE5250_PocketPC.zip` | Runs |  |
| [CEDBView](https://archive.org/details/tucows_38929_CEDBView) | `cedbview.zip` | Runs | Missing: `COREDLL.dll!CeFindFirstDatabase`, `COREDLL.dll!CeFindNextDatabase` |
| [CEniffer](https://archive.org/details/tucows_32190_CEniffer) | `CEnifferMIPS.zip.CAB` | Runs |  |
| [CETerm](https://archive.org/details/tucows_232362_CETerm) | `CETerm_PocketPC31.zip` | Runs |  |
| [CEVT220](https://archive.org/details/tucows_232369_CEVT220) | `CEVT220_PocketPC.zip` | Runs |  |
| [Chat](https://archive.org/details/tucows_32183_Chat) | `CHAT.zip` | Runs |  |
| [CodeWallet](https://archive.org/details/tucows_32399_CodeWallet) | `cwppcev101.exe` | Runs | Missing: `COREDLL.dll!SetScrollInfo`, `COREDLL.dll!GetScrollInfo`, `InkX.dll!InitInkX` |
| [CommLog](https://archive.org/details/tucows_358883_CommLog) | `CommLogCEDesktopSetup.zip` | Runs |  |
| [Connex](https://archive.org/details/tucows_304510_Connex) | `Connex_PPC_Setup.exe` | Runs |  |
| [CoolCalc](https://archive.org/details/tucows_32301_CoolCalc) | `coolcalcppc.exe` | Runs |  |
| [Crack Game](https://archive.org/details/tucows_309982_Crack_Game) | `Crack.exe` | Runs |  |
| [DeskClock](https://archive.org/details/tucows_32532_DeskClock) | `deskclck.zip` | Runs | Missing: `COREDLL.dll!PegHandleAppNotifications` |
| [DevilDarts](https://archive.org/details/tucows_291799_DevilDarts) | `DevilDartsSetup_demo_arm.exe` | Runs | Missing: `IMGDECMP.DLL!#1` |
| [Doodle Jump for Window Mobile 5.0](https://archive.org/details/doj_20251217) | `DOJ.zip` | Runs | Missing: `AYGSHELL.dll!#339` |
| [dotPocket](https://archive.org/details/tucows_270214_dotPocket) | `dotpocket2003.exe` | Runs |  |
| [Dragon Bane II](https://archive.org/details/tucows_273050_Dragon_Bane_II) | `DB2SetupARM.exe` | Runs |  |
| [dsFORTH](https://archive.org/details/tucows_32354_dsFORTH) | `ds2_b3_Setup.exe` | Runs |  |
| [dTree](https://archive.org/details/tucows_32496_dTree) | `dTree.exe` | Runs |  |
| [Elevens Game](https://archive.org/details/tucows_314055_Elevens_Game) | `Elevens.exe` | Runs |  |
| [ELFView - The Extremely Large File Viewer](https://archive.org/details/tucows_330232_ELFView_-_The_Extremely_Large_File_Viewer) | `ELFViewARM.CAB` | Runs | Missing: `COREDLL.dll!GetScrollInfo`, `COREDLL.dll!SetScrollInfo` |
| [Fedot's Adventures](https://archive.org/details/fedots-adventures) | `Fedot's Adventures.cab` | Runs |  |
| [Free Cell](https://archive.org/details/tucows_200089_Free_Cell) | `FreeCell_ARM.zip` | Runs |  |
| [Gem Slider](https://archive.org/details/tucows_250046_Gem_Slider) | `gslidece.exe` | Runs |  |
| [GNUChess](https://archive.org/details/tucows_309937_GNUChess) | `GNUChess.zip` | Runs | Missing: `COREDLL.dll!PtInRegion`, `COREDLL.dll!CreateRectRgnIndirect` |
| [Handy Paint](https://archive.org/details/handy-paint) | `HandyPaint.CAB` | Runs | Missing: `COREDLL.dll!CreateAcceleratorTableW` |
| [Hanoi Tower](https://archive.org/details/tucows_243676_Hanoi_Tower) | `CEZebra100b2.exe` | Runs | Missing: `COREDLL.dll!ferror` |
| [HPC Loan Manager](https://archive.org/details/tucows_32432_HPC_Loan_Manager) | `hpcloan.zip` | Runs | Missing: `COREDLL.dll!CeFindFirstDatabase`, `COREDLL.dll!CeFindNextDatabase`, `COREDLL.dll!CeOidGetInfo` |
| [HPC/PPC Stock Manager](https://archive.org/details/tucows_32412_HPC_PPC_Stock_Manager) | `hpcstock.zip` | Runs | Missing: `COREDLL.dll!CeOidGetInfo`, `COREDLL.dll!CeFindFirstDatabase`, `COREDLL.dll!CeFindNextDatabase` |
| [IntelliGOLF Par](https://archive.org/details/tucows_32414_IntelliGOLF_Par) | `igtparce.exe` | Runs | Missing: `COREDLL.dll!GetDateFormatW` |
| [IrDesk](https://archive.org/details/tucows_32505_IrDesk) | `IrDesk_All.zip` | Runs |  |
| [Klondike WAP Browser](https://archive.org/details/tucows_150885_Klondike_WAP_Browser) | `klondikewapsetup.exe` | Runs | Missing: `COREDLL.dll!SetScrollInfo`, `COREDLL.dll!sh__lshi64`, `COREDLL.dll!sh__divi64` |
| [Lemonade Tycoon](https://archive.org/details/lemonade-tycoon_202505) | `LemonadeTycoon.ARM.CAB` | Runs |  |
| [Line4Line](https://archive.org/details/tucows_210362_Line4Line) | `line4linearm.zip` | Runs | Missing: `COREDLL.dll!CheckMenuRadioItem` |
| [LinkSpy](https://archive.org/details/tucows_32202_LinkSpy) | `LinkSpySH3.zip` | Runs |  |
| [Loan Manager](https://archive.org/details/tucows_222365_Loan_Manager) | `pkpcloan_setup.exe` | Runs | Missing: `COREDLL.dll!CeFindFirstDatabase`, `COREDLL.dll!CeFindNextDatabase`, `COREDLL.dll!CeOidGetInfo` |
| [Locker](https://archive.org/details/tucows_328115_Locker) | `LockerCE_Trial.zip` | Runs |  |
| [Metalion ( Japan) ( ARM)](https://archive.org/details/metalion-japan-arm) | `Metalion (Japan) (ARM).zip` | Runs |  |
| [Metalion ( USA) ( ARM)](https://archive.org/details/metalion-usa-arm) | `Metalion (USA) (ARM).zip` | Runs |  |
| [MicroCalc Standard](https://archive.org/details/tucows_216317_MicroCalc_Standard) | `MicroCalc108.zip` | Runs |  |
| [miniBasic](https://archive.org/details/tucows_32364_miniBasic) | `minibasic.zip` | Runs | Missing: `COREDLL.dll!SetCaretPos`, `COREDLL.dll!CreateCaret`, `COREDLL.dll!ShowCaret` |
| [Moline](https://archive.org/details/tucows_222951_Moline) | `Moline_PocketPC_Trial.exe` | Runs | Missing: `IMGDECMP.DLL!#1` |
| [moongba2.4.2](https://archive.org/details/moongba2.4.2) | `moongba2.4.2.zip` | Runs | Missing: `COREDLL.dll!putchar` |
| [Nerve](https://archive.org/details/tucows_248733_Nerve) | `NerveDemoSetup.exe` | Runs |  |
| [PalmGB](https://archive.org/details/tucows_216709_PalmGB) | `pgbsetup.exe` | Runs | Missing: `COREDLL.dll!SelectPalette`, `COREDLL.dll!GetOpenFileNameW` |
| [PalmGB](https://archive.org/details/tucows_32646_PalmGB) | `pgbsetup.exe` | Runs | Missing: `COREDLL.dll!SelectPalette`, `COREDLL.dll!GetOpenFileNameW` |
| [PilotCE](https://archive.org/details/tucows_32648_PilotCE) | `PilotCE.exe` | Runs | Missing: `PilotCpuCE.dll!CPU_getlong`, `COREDLL.dll!GetVersionEx`, `PilotCpuCE.dll!CPU_pen` |
| [Pocket 12C](https://archive.org/details/tucows_255472_Pocket_12C) | `P12CV150ARM.exe` | Runs | Missing: `COREDLL.dll!CreatePenIndirect`, `COREDLL.dll!_ltow`, `COREDLL.dll!_fpclass` |
| [Pocket 15C Scientific Calculator](https://archive.org/details/tucows_294403_Pocket_15C_Scientific_Calculator) | `Pocket15CV110.exe` | Runs | Missing: `COREDLL.dll!CreatePenIndirect`, `COREDLL.dll!_fpclass`, `COREDLL.dll!_ltow` |
| [Pocket Pinball](https://archive.org/details/tucows_250021_Pocket_Pinball) | `PocketPinballSetupDemo.exe` | Runs |  |
| [Pocket Slay](https://archive.org/details/tucows_182765_Pocket_Slay) | `PocketSlay.zip` | Runs | Tap a tree in the top-left, then OK in the New Game dialog (the shareware version only lets you pick the first two rows). |
| [Pocket Taipei](https://archive.org/details/tucows_254025_Pocket_Taipei) | `ptaipei10.zip` | Runs | Missing: `OLEAUT32.dll!SystemTimeToVariantTime`, `OLEAUT32.dll!VariantTimeToSystemTime` |
| [Pocket Tune](https://archive.org/details/tucows_252624_Pocket_Tune) | `PocketTune_2_0_5.zip` | Runs | Missing: `COREDLL.dll!SetScrollRange`, `COREDLL.dll!SetScrollPos` |
| [Pocket Woman](https://archive.org/details/tucows_241852_Pocket_Woman) | `PocketWoman.zip` | Runs | Missing: `AYGSHELL.dll!#39` |
| [PocketConquest](https://archive.org/details/tucows_208847_PocketConquest) | `PocketConquest.zip` | Runs | Missing: `COREDLL.dll!SelectPalette` |
| [PocketFract](https://archive.org/details/tucows_196486_PocketFract) | `PocketFract.zip` | Runs | Missing: `AYGSHELL.dll!#88` |
| [PocketNav](https://archive.org/details/tucows_145763_PocketNav) | `pocketnav.zip` | Runs | Missing: `COREDLL.dll!DrawIconEx`, `COREDLL.dll!GetTimeFormatW` |
| [PocketNotepad](https://archive.org/details/tucows_145909_PocketNotepad) | `pocketnotepad21e.zip` | Runs |  |
| [PocketSTM](https://archive.org/details/tucows_213457_PocketSTM) | `pocketstmfstrarm.CAB` | Runs | Missing: `COREDLL.dll!_ltow`, `COREDLL.dll!DrawIconEx` |
| [Porta-Pinball](https://archive.org/details/tucows_219512_Porta-Pinball) | `PortaPinSetup300.exe` | Runs |  |
| [PPC Planet CE Archive](https://archive.org/details/ppc-planet-ce) | `ClockWizard.rar` | Runs |  |
| [Pretec CompactGPS](https://archive.org/details/compact-gps-lp) | `CompactGPS.zip` | Runs |  |
| [Pretec CompactGPS](https://archive.org/details/compact-gps-lp) | `CompactGPS_LP.zip` | Runs |  |
| [Primer](https://archive.org/details/tucows_76856_Primer) | `PrimerCE.exe` | Runs | Missing: `COREDLL.dll!EnumFontFamiliesW`, `COREDLL.dll!IntersectClipRect`, `COREDLL.dll!SelectClipRgn` |
| [SEdit](https://archive.org/details/tucows_180072_SEdit) | `seditmips.zip` | Runs |  |
| [Seven Game](https://archive.org/details/tucows_312195_Seven_Game) | `Seven.exe` | Runs |  |
| [SnoopSoft Dashboard](https://archive.org/details/tucows_216191_SnoopSoft_Dashboard) | `dashboard.zip` | Runs | Missing: `COREDLL.dll!CompareStringW` |
| [Soccer Kid Folder](https://archive.org/details/soccer-kid-folder) | `Soccer Kid Folder.zip` | Runs |  |
| [Sorcery PDA](https://archive.org/details/tucows_308748_Sorcery_PDA) | `InstallDXA.exe` | Runs |  |
| [SpreadCE for Windows CE 3.12 2022-08-03](https://archive.org/details/spreadce-312-20220803) | `PPC/spreppcs.zip` | Runs | Missing: `COREDLL.dll!sh__itoi64`, `COREDLL.dll!sh__muli64`, `COREDLL.dll!sh__divi64` |
| [Stock Manager](https://archive.org/details/tucows_222355_Stock_Manager) | `pkpcstocks_setup.exe` | Runs | Missing: `COREDLL.dll!CeOidGetInfo`, `COREDLL.dll!CeFindFirstDatabase`, `COREDLL.dll!CeFindNextDatabase` |
| [TZO Dynamic DNS](https://archive.org/details/tucows_277018_TZO_Dynamic_DNS) | `tzopocketpc.zip` | Runs | Missing: `WINSOCK.dll!WSAStartup`, `WINSOCK.dll!closesocket` |
| [UniversalRemoteCE](https://archive.org/details/tucows_245237_UniversalRemoteCE) | `UniversalRemote_trial.exe` | Runs |  |
| [VoiceRecorder](https://archive.org/details/tucows_289072_VoiceRecorder) | `VoiceRecorderSetup_TRIAL_tucows.exe` | Runs | Missing: `COREDLL.dll!acmDriverEnum`, `COREDLL.dll!acmDriverOpen`, `COREDLL.dll!acmMetrics` |
| [Windows CE 1.0 Programs](https://archive.org/details/windowsce1.0) | `Windows CE 1.0.zip` | Runs | Missing: `COREDLL.dll!AudioUpdateFromRegistry` |
| [Windows mobile cabs](https://archive.org/details/tombraider_arm_hexturbation) | `line4setup.arm.CAB` | Runs | Missing: `COREDLL.dll!CheckMenuRadioItem` |
| [WinEyes](https://archive.org/details/tucows_226355_WinEyes) | `wineyesce.zip` | Runs |  |
| [XLISP-Plus](https://archive.org/details/tucows_32693_XLISP-Plus) | `xl21gce.zip` | Runs | Missing: `COREDLL.dll!IntersectClipRect` |
| [Yotz](https://archive.org/details/tucows_32181_Yotz) | `yotzdemo.exe` | Runs |  |

## 🟡 Partly works (156)

Imports and starts, but the screen stays blank, draws nothing, or stops soon.

| Title | File | Result | Notes |
|---|---|---|---|
| [A few Games, Software and Themes for PocketPC](https://archive.org/details/game_20260818) | `Game.rar` | Blank screen | Missing: `DocList.dll!DocList_Create`, `AYGSHELL.dll!#88` |
| [AI Baseball](https://archive.org/details/tucows_249836_AI_Baseball) | `aibaseballdemo_100.zip` | Exits at once |  |
| [AI Football](https://archive.org/details/tucows_223136_AI_Football) | `aifootballdemo_120.zip` | Exits at once |  |
| [ArCEnoid](https://archive.org/details/tucows_32070_ArCEnoid) | `ArCEnoid_9.zip` | Stops later | The game stopped on something the emulator cannot do yet. Missing: `COREDLL.dll!MaskBlt` |
| [Arkanoi 3 D PPC WM 5 V 1 0](https://archive.org/details/arkanoi-3-d-ppc-wm-5-v-1-0) | `Arkanoi3D_PPC_WM5_v1_0.CAB` | Exits at once | Missing: `COREDLL.dll!GetFileAttributesExW` |
| [AT&T PocketPC/Windows Mobile Game Collection (EN)](https://archive.org/details/games_202405) | `Games.zip` | Exits at once |  |
| [Better Measure](https://archive.org/details/tucows_262702_Better_Measure) | `Bettermeasuredemo.zip` | Exits at once |  |
| [Blaster](https://archive.org/details/tucows_234823_Blaster) | `BlasterDemo.zip` | No frames |  |
| [Book Bag](https://archive.org/details/tucows_251297_Book_Bag) | `bookbag_trial_pocket.exe` | Exits at once |  |
| [Bubblets](https://archive.org/details/tucows_32120_Bubblets) | `bubblets.zip` | Exits at once | The game stopped on something the emulator cannot do yet. |
| [BugLord 1.0](https://archive.org/details/buglord-1.0) | `for_Pocket_PC_PPC.STRONGARM.cab` | No frames | Missing: `IMGDECMP.DLL!#1` |
| [buglord-pocketpc-1.00](https://archive.org/details/buglord-pocketpc-1.00) | `for_Pocket_PC_PPC.STRONGARM.cab` | No frames | Missing: `IMGDECMP.DLL!#1` |
| [CE6b](https://archive.org/details/tucows_52638_CE6b) | `ce6b_hpc.zip` | Stops later |  |
| [CEBall](https://archive.org/details/tucows_32077_CEBall) | `CEBall_All.zip` | Blank screen |  |
| [CEWedge](https://archive.org/details/tucows_163947_CEWedge) | `cewedge.exe` | Exits at once |  |
| [CEWedge](https://archive.org/details/tucows_259055_CEWedge) | `cewedge.exe` | Exits at once |  |
| [Chinese Input](https://archive.org/details/tucows_32573_Chinese_Input) | `cimesh.zip` | Blank screen | Missing: `COREDLL.dll!GetVersionEx`, `COREDLL.dll!Shell_NotifyIcon` |
| [CHM eBook Reader](https://archive.org/details/tucows_371821_CHM_eBook_Reader) | `chm_ebook_reader_trial.zip` | Stops later | The game needs a file that is not in the package. Missing: `AYGSHELL.dll!#188`, `COREDLL.dll!Shell_NotifyIcon`, `COREDLL.dll!GetTempPathW` |
| [CnetX Pocket Slideshow 1.51 for Windows CE (Extracted)](https://archive.org/details/ss151aze-extracted) | `SS.PROARM.CAB` | Exits at once | The game needs a file that is not in the package. Missing: `ole32.dll!StringFromGUID2`, `htmlview.dll!InitHTMLControl`, `COREDLL.dll!GetDlgItemTextW` |
| [CnetX Pocket Slideshow 1.51 for Windows CE (Extracted)](https://archive.org/details/ss151aze-extracted) | `SS.H2KARM.CAB` | Exits at once | The game needs a file that is not in the package. Missing: `ole32.dll!StringFromGUID2`, `htmlview.dll!InitHTMLControl`, `COREDLL.dll!GetDlgItemTextW` |
| [COD 2 Windows Mobile (3D-accelerated)](https://archive.org/details/cod-2-WM3D) | `COD2SOInstaller.cab` | Stops later | The game stopped on something the emulator cannot do yet. |
| [Conduits Copilot CE for Palm-size PCs](https://archive.org/details/pilotce-conduits) | `pilotce.zip` | Exits at once | Missing: `pilotcpu.dll!CPU_init` |
| [CoverMe](https://archive.org/details/tucows_32484_CoverMe) | `CoverMePro.exe` | Stops later |  |
| [CreativeNotes](https://archive.org/details/tucows_377348_CreativeNotes) | `cnsetupv120.exe` | No frames | Missing: `richink.dll!InitRichInkDLL`, `InkX.dll!InitInkX` |
| [Crosswords](https://archive.org/details/tucows_372047_Crosswords) | `xwords_406.zip` | Exits at once |  |
| [CVsuite](https://archive.org/details/CVsuite) | `CVsuite.zip` | Exits at once |  |
| [Decuma](https://archive.org/details/Decuma) | `Decuma.zip` | Blank screen | Missing: `COREDLL.dll!GetMouseMovePoints`, `COREDLL.dll!CreateRectRgn` |
| [Decuma OnSpot for Pocket PC and Palm OS](https://archive.org/details/decuma-onspot-ppc-palmos) | `decuma_onspot_handwriting_recognition_3_01_ppc.exe` | Blank screen | Missing: `COREDLL.dll!GetMouseMovePoints`, `COREDLL.dll!CreateRectRgn` |
| [Dell Axim X5 Pocket PC 2003 Upgrade CD](https://archive.org/details/wmx5dell) | `wmx5dell.rar` | Exits at once |  |
| [DialupMaster](https://archive.org/details/tucows_32191_DialupMaster) | `DialupMaster100.zip` | No frames | Missing: `COREDLL.dll!RasEnumConnections`, `COREDLL.dll!Shell_NotifyIcon` |
| [drivers](https://archive.org/details/drivers_forsocketandsymboleWLANwm5PPC) | `drivers.zip` | Blank screen | Missing: `CEWDM.dll!CEDeviceIoControl`, `CEWDM.dll!CECreateFile` |
| [dTree](https://archive.org/details/tucows_183459_dTree) | `idTree.exe` | No frames |  |
| [E6 Calculator](https://archive.org/details/tucows_32304_E6_Calculator) | `e6demo.zip` | Stops later |  |
| [Fade (Version 1.04, French) [Pocket PC]](https://archive.org/details/fade-1.04-fr-pocket-pc) | `Fade (1.04 FR).exe` | Exits at once | The game needs a file that is not in the package. |
| [Fade (Version 1.06 patch, English) [Pocket PC]](https://archive.org/details/fade-1.06-pocket-pc) | `Fade Patch 1.04 to 1.06 US.zip` | Exits at once | The game needs a file that is not in the package. |
| [Ferrari GT: Evolution HD+ for HTC HD2 and Toshiba Tsunagi](https://archive.org/details/ferrari-gt-hd2-wm) | `Ferrari_GT_HD2_Demo_MOD.cab` | No frames | Missing: `toolhelp.dll!CreateToolhelp32Snapshot`, `toolhelp.dll!Process32First`, `toolhelp.dll!CloseToolhelp32Snapshot` |
| [File Look](https://archive.org/details/tucows_182994_File_Look) | `filelook_arm.zip` | Exits at once |  |
| [Filegram](https://archive.org/details/tucows_32499_Filegram) | `Filegram100-ARM.zip` | Stops later |  |
| [Flower Power](https://archive.org/details/tucows_216347_Flower_Power) | `fpowv10demo.zip` | Exits at once |  |
| [Four Corners](https://archive.org/details/tucows_32130_Four_Corners) | `4Corners_HPC.exe` | Exits at once | The game stopped on something the emulator cannot do yet. Missing: `COREDLL.dll!ImageList_Duplicate` |
| [Fun Pocket PC Expansion Pack](https://archive.org/details/FunPocketPCExpansionPack) | `Fun Pocket PC Expansion Pack.zip` | Stops later | The game stopped on something the emulator cannot do yet. |
| [Globe](https://archive.org/details/tucows_200221_Globe) | `GlobePocketPCARM.exe` | Exits at once | The game stopped on something the emulator cannot do yet. |
| [GPSeasyCE](https://archive.org/details/tucows_199278_GPSeasyCE) | `GpsARM.exe` | Blank screen |  |
| [HandyCash](https://archive.org/details/tucows_146871_HandyCash) | `HandyCash.zip` | Exits at once |  |
| [HP Jornada 720 jLime](https://archive.org/details/hp-jornada-720-jlime) | `jlinexec.exe` | No frames | Missing: `COREDLL.dll!VirtualCopy` |
| [HTML Editor](https://archive.org/details/tucows_32361_HTML_Editor) | `html_hpc.zip` | Exits at once | The game needs a file that is not in the package. Missing: `htmlview.dll!InitHTMLControl` |
| [I Presentation Mobile Player ARM](https://archive.org/details/i-presentation-mobile-player-arm) | `iPresentation Mobile Player ARM.zip` | Exits at once | Missing: `coredll.dll!LoadLibraryExW` |
| [Internal Specsavers software (stock management) for Windows mobile](https://archive.org/details/specsavers-hht) | `Specsavers HHT.zip` | Exits at once |  |
| [JETCET PRINT](https://archive.org/details/tucows_32283_JETCET_PRINT) | `jetcet.exe` | Exits at once |  |
| [Jimmy Quick Menu](https://archive.org/details/tucows_32285_Jimmy_Quick_Menu) | `JQMSH3.ZIP` | Blank screen | Missing: `COREDLL.dll!Shell_NotifyIcon` |
| [K-ForCE](https://archive.org/details/tucows_210588_K-ForCE) | `K-ForCE.v10S.zip` | No frames | Missing: `COREDLL.dll!SetScrollInfo`, `COREDLL.dll!SetScrollPos`, `COREDLL.dll!CreatePatternBrush` |
| [M3 Mobile OX10 drivers (CE and WM)](https://archive.org/details/M3_OX10) | `M3OX10_WM_FlashDisk.zip` | Exits at once |  |
| [MapFactor Free 21 Windows CE and Windows NT](https://archive.org/details/mapfactorfree21) | `MapFactor21/Install/pnd_app.rar` | Exits at once | Missing: `COREDLL.dll!fwprintf`, `COREDLL.dll!GetTempPathW`, `COREDLL.dll!LoadLibraryExW` |
| [Microsoft Hearts for Pocket PC & Windows Mobile](https://archive.org/details/hearts_202601) | `Hearts.cab` | Exits at once |  |
| [Microsoft Money 2003 for Pocket PC](https://archive.org/details/microsoft-money-2003-for-pocket-pc) | `arm720.cab` | Exits at once |  |
| [Microsoft Money 2003 for Pocket PC](https://archive.org/details/microsoft-money-2003-for-pocket-pc) | `sa1100.cab` | Exits at once |  |
| [Microsoft Money 2004 for Pocket PC](https://archive.org/details/microsoft-money-2004-for-pocket-pc) | `arm720.cab` | Exits at once |  |
| [Microsoft Money 2004 for Pocket PC](https://archive.org/details/microsoft-money-2004-for-pocket-pc) | `sa1100.cab` | Exits at once |  |
| [Microsoft Money 2005 for Pocket PC](https://archive.org/details/microsoft-money-2005-for-pocket-pc) | `sa1100.cab` | Exits at once |  |
| [Microsoft Money for Handheld PC 1.0](https://archive.org/details/microsoft-money-for-handheld-pc-1.0) | `HPCArmP.CAB` | Exits at once |  |
| [Microsoft Money for Handheld PC 1.0](https://archive.org/details/microsoft-money-for-handheld-pc-1.0) | `HPCMips.CAB` | Exits at once |  |
| [Microsoft Pocket Streets 2002 English GPS Update (Extracted)](https://archive.org/details/p.-rap-mips) | `P.RAP_ARM.CAB` | Exits at once |  |
| [Microsoft Pocket Streets for Pocket PC 2005 (Extracted)](https://archive.org/details/pocket-streets-2005-cabs) | `Microsoft Pocket Streets for Pocket PC - 2005/PocketStreets.PPCArm.CAB` | Exits at once | Missing: `COREDLL.dll!GetOpenFileNameW` |
| [Microsoft Pocket Streets for Pocket PC 2006 (Extracted)](https://archive.org/details/pocket-streets-2006-pocketpc) | `Microsoft Pocket Streets for Pocket PC/PocketStreets.PPCArm.CAB` | Exits at once | Missing: `COREDLL.dll!GetOpenFileNameW` |
| [Mobile Administrator](https://archive.org/details/tucows_77262_Mobile_Administrator) | `madmin.zip` | Stops later | The game needs a file that is not in the package. |
| [Multi Window Notepad](https://archive.org/details/tucows_209428_Multi_Window_Notepad) | `mwnsetup.exe` | Exits at once |  |
| [Multi Window Notepad](https://archive.org/details/tucows_209429_Multi_Window_Notepad) | `mwnsetup.exe` | Exits at once |  |
| [My Books](https://archive.org/details/tucows_222632_My_Books) | `MyBooks_Trial_Version10.zip` | Exits at once |  |
| [MyMobiler 1.2.5 for Windows Mobile](https://archive.org/details/MyMobiler) | `MyMobiler_WM_1.2.5.zip` | No frames | Missing: `AYGSHELL.dll!#339`, `COREDLL.dll!Shell_NotifyIcon` |
| [nPOPuk 3.05 Windows Mobile/Windows CE](https://archive.org/details/npopuk3.05) | `npopuk305_wce50arm (1).zip` | Exits at once | The game needs a file that is not in the package. Missing: `COREDLL.dll!ImageList_ReplaceIcon`, `COREDLL.dll!DestroyIcon`, `COREDLL.dll!ImageList_SetOverlayImage` |
| [nPOPuk 3.05 Windows Mobile/Windows CE](https://archive.org/details/npopuk3.05) | `npopuk305ssl_wce50arm.zip` | Exits at once | The game needs a file that is not in the package. Missing: `COREDLL.dll!ImageList_ReplaceIcon`, `COREDLL.dll!DestroyIcon`, `COREDLL.dll!ImageList_SetOverlayImage` |
| [OmniSolve](https://archive.org/details/tucows_220911_OmniSolve) | `OmniSolvePPC_Install.exe` | Exits at once | The game stopped on something the emulator cannot do yet. |
| [Outliner](https://archive.org/details/tucows_32421_Outliner) | `hpcoutliner.zip` | Exits at once | The game stopped on something the emulator cannot do yet. Missing: `COREDLL.dll!RegisterClipboardFormatW` |
| [PalmFlash](https://archive.org/details/tucows_32645_PalmFlash) | `PalmFlash.exe` | Blank screen |  |
| [Panda Vs. UFO](https://archive.org/details/tucows_78647_Panda_Vs._UFO) | `PandaVSUFO_Trial_PPC_SH3.exe` | No frames |  |
| [ParaWinCE](https://archive.org/details/tucows_214659_ParaWinCE) | `pg50101t.exe` | Exits at once |  |
| [pDiveLog](https://archive.org/details/tucows_302095_pDiveLog) | `pDiveLogPPC300Setup.exe` | Blank screen | Missing: `InkX.dll!InitInkX`, `COREDLL.dll!CreateRectRgn` |
| [PHM registry editor v0.70](https://archive.org/details/phm-registry-editor.0.70) | `phm-registry-editor.0.70.cab` | No frames | Missing: `COREDLL.dll!RegisterClipboardFormatW` |
| [phoneME Windows CE rev20547 2013-08-15 cabs](https://archive.org/details/phoneme-windows-ce-rev20547-2013-08-15-cabs) | `CLDC-phoneme_feature_mr4-win32_arm_wm5-bin-rev20547-20130815.cab` | Blank screen |  |
| [phoneME Windows CE rev20547 2013-08-15 cabs](https://archive.org/details/phoneme-windows-ce-rev20547-2013-08-15-cabs) | `CLDC-phoneme_feature_mr4-win32_arm_wm6-bin-rev20547-20130815.cab` | Blank screen |  |
| [PhoneTone](https://archive.org/details/tucows_222934_PhoneTone) | `PhoneTonePKT30Day.exe` | Exits at once | The game needs a file that is not in the package. |
| [Pico Drive (GX.DDL INCLUDED!!)](https://archive.org/details/pico-drive_202607) | `PicoDrive.exe` | No frames | Missing: `COREDLL.dll!GetOpenFileNameW` |
| [Pocket Artist](https://archive.org/details/tucows_47245_Pocket_Artist) | `PocketArtist10.exe` | Stops later | The game stopped on something the emulator cannot do yet. Missing: `COREDLL.dll!SetFileTime`, `COREDLL.dll!GetMouseMovePoints` |
| [Pocket BizCalc Pocket PC](https://archive.org/details/Pocket-BizCalc-1999-02-22-ce-ppc) | `pbizcalc.sh3.CAB` | Blank screen |  |
| [Pocket Chord Finder](https://archive.org/details/tucows_232040_Pocket_Chord_Finder) | `pcf15.zip` | Exits at once |  |
| [Pocket Cool-O-Meter](https://archive.org/details/tucows_145485_Pocket_Cool-O-Meter) | `pocketcm.zip` | Exits at once |  |
| [Pocket Explorer](https://archive.org/details/tucows_212439_Pocket_Explorer) | `pexplorerarm.exe` | Blank screen |  |
| [Pocket Explorer](https://archive.org/details/tucows_212439_Pocket_Explorer) | `pexplorersh.exe` | Blank screen |  |
| [Pocket Genesis](https://archive.org/details/pocket-genesis_202607) | `PocketGenesis.exe` | Blank screen |  |
| [Pocket Genesis 1.0](https://archive.org/details/pocketgenesis1.0b11) | `PocketGenesis1.0b1arm.zip` | Blank screen |  |
| [Pocket Genesis 1.0](https://archive.org/details/pocketgenesis1.0b11) | `PocketGenesis1.0b2-38xx.zip` | Blank screen |  |
| [Pocket Hockey - 2004](https://archive.org/details/tucows_332850_Pocket_Hockey_-_2004) | `pkHockey.exe` | Stops later | Missing: `COREDLL.dll!GetDateFormatW`, `OLEAUT32.dll!SystemTimeToVariantTime`, `COREDLL.dll!_ecvt` |
| [Pocket Music Box Demo ( ARM)](https://archive.org/details/pocket-music-box-demo-arm) | `Pocket Music Box Demo (ARM).zip` | Exits at once | The game stopped on something the emulator cannot do yet. Missing: `COREDLL.dll!GetClipBox` |
| [Pocket PC software](https://archive.org/details/spb-polish-cards.arm) | `pocket pc app/7z920-arm.cab` | No frames | Missing: `COREDLL.dll!GetDialogBaseUnits` |
| [Pocket PC Teletype GPS Full](https://archive.org/details/appspocketpcteletypegpsfull) | `(Apps) - Pocket PC - Teletype GPS Full.zip` | Blank screen | Missing: `COREDLL.dll!GetClipBox`, `COREDLL.dll!MaskBlt`, `GPSDriver.dll!?TTGPS_GetPositionalData@@YA_NPAUTTGPS_POSITIONALDATA@@@Z` |
| [Pocket Quake II](https://archive.org/details/tucows_315702_Pocket_Quake_II) | `pocketquake2.exe` | Exits at once | The game needs a file that is not in the package. Missing: `WINSOCK.dll!WSAStartup` |
| [Pocket Rockets Pro](https://archive.org/details/tucows_328941_Pocket_Rockets_Pro) | `PRP_Trial.zip` | No frames | Missing: `gd201b.dll!?BltFast@CGapiSurface@@QAAJKKPAV1@PAUtagRECT@@KPAU_GDBLTFASTFX@@@Z`, `gd201b.dll!?Flip@CGapiDisplay@@QAAJXZ`, `gd201b.dll!?WaitForNextFrame@CGapiTimer@@QAAJXZ` |
| [Pocket Solitaire Engine](https://archive.org/details/tucows_301930_Pocket_Solitaire_Engine) | `SolitaireEngineDemo.exe` | Exits at once |  |
| [Pocket Spark Handheld PC (Extracted)](https://archive.org/details/pocket-spark-hpc) | `PocketSparkHPC.HPC2K_ARM.CAB` | Blank screen |  |
| [Pocket Spark Handheld PC (Extracted)](https://archive.org/details/pocket-spark-hpc) | `PocketSparkHPC.HPC2K_mips.CAB` | Blank screen |  |
| [PocketCDG](https://archive.org/details/tucows_359704_PocketCDG) | `pocketcdg-100-unregistered-setup.exe` | No frames | Missing: `GD104b.dll!?DrawTextW@CGapiSurface@@QAAJKKPBGPAVCGapiBitmapFont@@KPAU_GDTEXTFX@@PAK@Z`, `GD104b.dll!?DrawLine@CGapiSurface@@QAAJKKKKKKPAU_GDLINEFX@@@Z`, `GD104b.dll!?FillRect@CGapiSurface@@QAAJPAUtagRECT@@KKPAU_GDFILLRECTFX@@@Z` |
| [PocketPad](https://archive.org/details/tucows_145908_PocketPad) | `pocketpad.zip` | Exits at once |  |
| [PocketPixPrint](https://archive.org/details/tucows_173478_PocketPixPrint) | `PocketPixPrint_Install.exe` | Exits at once |  |
| [PocketSuspendFX](https://archive.org/details/tucows_300345_PocketSuspendFX) | `PocketSuspendFXSetup.exe` | Exits at once |  |
| [Poker Sol](https://archive.org/details/tucows_32166_Poker_Sol) | `PokerSol_HPC.exe` | Exits at once | The game stopped on something the emulator cannot do yet. |
| [PowerTime](https://archive.org/details/tucows_32539_PowerTime) | `PT11ev.exe` | Exits at once | Missing: `COREDLL.dll!GetVersionEx` |
| [PowerTime](https://archive.org/details/tucows_32540_PowerTime) | `PT11ev.exe` | Exits at once | Missing: `COREDLL.dll!GetVersionEx` |
| [PowerVR Demos](https://archive.org/details/pvr-racer) | `OGLESParticles.zip` | Stops later | Missing: `COREDLL.dll!MonitorFromWindow`, `COREDLL.dll!GetMonitorInfo` |
| [PPToools](https://archive.org/details/tucows_199958_PPToools) | `pptools.zip` | Stops later | Missing: `CSFileView.dll!#1` |
| [ProgramMenu](https://archive.org/details/tucows_183690_ProgramMenu) | `pmenu14.zip` | Blank screen | Missing: `AYGSHELL.dll!#2` |
| [ProxiWeb](https://archive.org/details/tucows_32654_ProxiWeb) | `PrxWebCE3B1.exe` | No frames | Missing: `WINSOCK.dll!__WSAFDIsSet`, `WINSOCK.dll!select`, `WINSOCK.dll!htonl` |
| [PTE](https://archive.org/details/tucows_222772_PTE) | `ptef.zip` | Exits at once |  |
| [QISndFx](https://archive.org/details/tucows_32373_QISndFx) | `QISndFx_SH3.zip` | Stops later |  |
| [QRemote](https://archive.org/details/tucows_216286_QRemote) | `qr_arm.cab` | Exits at once |  |
| [Quake 3 PocketPC [q3ce-1.0]](https://archive.org/details/q3ce-1.0-release-armv4ppc2003) | `q3ce-1.0-release-armv4ppc2003.exe` | Blank screen |  |
| [racePro](https://archive.org/details/tucows_32437_racePro) | `racePro.zip` | Stops later |  |
| [Registry Explorer](https://archive.org/details/tucows_32379_Registry_Explorer) | `RExplrhCE.exe` | Stops later | Missing: `COREDLL.dll!ImageList_ReplaceIcon`, `COREDLL.dll!CreatePatternBrush`, `COREDLL.dll!GetDlgItemTextW` |
| [ROMExtractor](https://archive.org/details/ROMExtractor) | `ROMExtractor.exe` | Exits at once | Missing: `coredll.dll!SetProcPermissions`, `coredll.dll!KernelLibIoControl` |
| [S-Calc](https://archive.org/details/tucows_230457_S-Calc) | `Pocket_S-Calc.zip` | Exits at once |  |
| [Sharp EM ONE Update Files](https://archive.org/details/sharp-em-one-update-files) | `Sharp EM-One Updates.zip` | Exits at once |  |
| [Shutdown](https://archive.org/details/tucows_145463_Shutdown) | `shutdownppc.zip` | Stops later |  |
| [SolPack 2 (HPC)](https://archive.org/details/tucows_32174_SolPack_2_HPC) | `SP2DemoSh3.exe` | Exits at once |  |
| [Starbuck](https://archive.org/details/tucows_32617_Starbuck) | `SBInstall.exe` | Exits at once | Missing: `COREDLL.dll!EnumFontFamiliesW` |
| [Stardock PocketBlinds v1.0 ARM](https://archive.org/details/pb10-arm) | `pb10-arm.cab` | No frames | Missing: `coredll.dll!IsAPIReady` |
| [Startler](https://archive.org/details/tucows_32543_Startler) | `StInstall.exe` | Exits at once | Missing: `COREDLL.dll!CeRunAppAtTime` |
| [Stealth GPS](https://archive.org/details/tucows_246238_Stealth_GPS) | `StealthGPS.zip` | Exits at once |  |
| [STRegedit](https://archive.org/details/tucows_305971_STRegedit) | `stexplorer_arm.zip` | Stops later | Missing: `COREDLL.dll!GetDateFormatW`, `COREDLL.dll!CompareFileTime` |
| [Stupid PIM Tricks](https://archive.org/details/tucows_32443_Stupid_PIM_Tricks) | `CEBall_sh3_hpc.zip` | Blank screen |  |
| [Symbol MC50 mc50_ppc2003_wm5_0_upgrade_r1.4](https://archive.org/details/mc50_ppc2003_wm5_0_upgrade_r1.4) | `mc50_ppc2003_wm5_0_upgrade_r1.4.zip` | Exits at once |  |
| [Tascal RegEdit](https://archive.org/details/tucows_32381_Tascal_RegEdit) | `reg050m.zip` | Blank screen | Missing: `COREDLL.dll!ImageList_ReplaceIcon`, `COREDLL.dll!DestroyIcon`, `COREDLL.dll!GetScrollInfo` |
| [TCPMP Core Desktop Installer Extracted](https://archive.org/details/tcpmp-cabs) | `TCPMP/tcpmp.arm_ce2.cab` | Exits at once |  |
| [Technician Assistant](https://archive.org/details/tucows_50861_Technician_Assistant) | `TAppc11.zip` | Exits at once |  |
| [Terminal Services Client for Pocket PC 2002](https://archive.org/details/terminal-services-client-for-pocket-pc-2002) | `tsc.PPC_SA1100.CAB` | Exits at once | Missing: `AYGSHELL.dll!#43` |
| [Tested software for Windows Mobile 2003](https://archive.org/details/ppc-2003) | `PocketGnuboy/pgnuboy.ppc3_arm.CAB` | Blank screen |  |
| [TextMaker](https://archive.org/details/tucows_315355_TextMaker) | `tmpw02ev.exe` | Exits at once |  |
| [TimeTracker 2000](https://archive.org/details/tucows_78965_TimeTracker_2000) | `TTT.PPC.MIPS.exe` | Exits at once |  |
| [TrayDate](https://archive.org/details/tucows_32627_TrayDate) | `TrayDate.exe` | Blank screen | Missing: `COREDLL.dll!Shell_NotifyIcon`, `COREDLL.dll!DestroyIcon`, `COREDLL.dll!CreateIconIndirect` |
| [UKEditor](https://archive.org/details/UKEditor) | `UKEditor.zip` | No frames | Missing: `COREDLL.dll!ExtractIconExW` |
| [Ultima Underworld for Pocket PC](https://archive.org/details/ultima-underworld-for-pocket-pc) | `Ultima Underworld PPC cabs -iTLcCc-.zip` | Exits at once |  |
| [Unfinished Business](https://archive.org/details/unfinished-business_202610) | `Unfinished Business.zip` | Exits at once | The game stopped on something the emulator cannot do yet. Missing: `alib.dll!#22`, `alib.dll!#19`, `alib.dll!#20` |
| [Update WS 011 SH SP 5a](https://archive.org/details/update-ws-011-sh-sp-5a) | `Update-WS011SH-SP5a.exe` | Exits at once |  |
| [VBirthday GOLD Edition](https://archive.org/details/tucows_265958_VBirthday_GOLD_Edition) | `VBirthday21GoldTrialSetup.exe` | Exits at once | The game needs a file that is not in the package. |
| [vxPage](https://archive.org/details/tucows_32689_vxPage) | `vxPage.zip` | Exits at once | Missing: `COREDLL.dll!EnumFontFamiliesW`, `COREDLL.dll!_ltow`, `COREDLL.dll!lineInitialize` |
| [vxSniffer](https://archive.org/details/tucows_283090_vxSniffer) | `vxSniffer.zip` | Exits at once | Missing: `COREDLL.dll!RegisterDevice` |
| [vxWeb](https://archive.org/details/tucows_32232_vxWeb) | `vxWeb.zip` | Exits at once | Missing: `WINSOCK.dll!WSAStartup` |
| [wincecabs3](https://archive.org/details/wincecabs3) | `Pocket MBA-Calc/pmba.arm.CAB` | Exits at once | The game stopped on something the emulator cannot do yet. Missing: `COREDLL.dll!_fcvt` |
| [Windows Media Player for Pocket PC - Technology Preview Edition 2](https://archive.org/details/windowsmediaplayerforpocketpc-technologypreviewedition12extracted) | `Windows Media Player for Pocket PC - Technology Preview Edition/WMP.PPC_SA1100.CAB` | Exits at once | Missing: `COREDLL.dll!UnregisterClassW` |
| [Windows Mobile application archive](https://archive.org/details/windowsmobile) | `GPRSMonitor_PPC12.cab` | No frames | Missing: `COREDLL.dll!CompareFileTime` |
| [Windows mobile cabs](https://archive.org/details/tombraider_arm_hexturbation) | `psc.ARM.CAB` | Exits at once |  |
| [WinMobile Lens](https://archive.org/details/tucows_371685_WinMobile_Lens) | `WinMobileLens_SetupADS_Arm.exe` | Blank screen |  |
| [WinPopup](https://archive.org/details/tucows_32691_WinPopup) | `winpopupce.zip` | Exits at once |  |
| [Word Search](https://archive.org/details/tucows_32179_Word_Search) | `WSDemo.exe` | Exits at once |  |
| [Xylon GPS Data logging](https://archive.org/details/tucows_227730_Xylon_GPS_Data_logging) | `Xylon_Trial.zip` | Exits at once |  |
| [Yacht](https://archive.org/details/tucows_168569_Yacht) | `Yacht.zip` | Exits at once |  |
| [ZIO Simcity 2000](https://archive.org/details/zio-simcity-2000) | `ZIO_Simcity_2000 v1.00.cab` | Stops later | Missing: `COREDLL.dll!CreatePenIndirect`, `COREDLL.dll!GetPaletteEntries` |
| [ZIO Simcity 2000](https://archive.org/details/zio-simcity-2000) | `ZIO Simcity 2000 v1.01 Patched Scrollbars.cab` | Stops later | Missing: `COREDLL.dll!CreatePenIndirect`, `COREDLL.dll!GetPaletteEntries` |

## 🟥 Does not start (390)

Imports, but fails while starting (usually a missing system DLL or an API not emulated yet).

| Title | File | Result | Notes |
|---|---|---|---|
| [.net Compact Frame Work 1.0 Sp 3](https://archive.org/details/net-compact-frame-work-1.0-sp-3) | `.net compact frame work 1.0 sp3.msi.zip` | Does not start | This is a desktop Windows program, not a Pocket PC one. |
| [.NET Compact Framework 2.0 (& SR)](https://archive.org/details/netcompact2.0) | `NETCFv2.wm.armv4i.cab` | Does not start | This is a .NET Compact Framework program, which PocketPC3630 cannot run. |
| [.NET Compact Framework 2.0 (& SR)](https://archive.org/details/netcompact2.0) | `NETCFv2.wce5.armv4i.cab` | Does not start |  |
| [2nR-Musiker Wireless MP3 Client and Server](https://archive.org/details/tucows_217288_2nR-Musiker_Wireless_MP3_Client_and_Server) | `2nR-ipaq.zip` | Does not start | The game needs a file that is not in the package. Missing: `WINSOCK.dll!socket`, `WINSOCK.dll!inet_addr`, `WINSOCK.dll!WSAStartup` |
| [Abacus.pocketpc 2002 Arm](https://archive.org/details/abacus.pocketpc-2002-arm) | `Abacus.pocketpc2002_arm.zip` | Does not start | The game stopped on something the emulator cannot do yet. |
| [Adobe Acrobat Reader for Pocket PC 1.0](https://archive.org/details/adobe-acrobat-reader-for-pocket-pc-1.0) | `arceARMr.PPC_2577.cab` | Does not start | The game needs a file that comes with the Pocket PC itself. Missing: `mfcce300.dll!#644` |
| [Adobe Acrobat Reader for Pocket PC 2.0 (ARM Only CAB)](https://archive.org/details/arceARMen20) | `arceARMen_usr.PPC_2577.CAB` | Does not start | The game needs a file that comes with the Pocket PC itself. Missing: `mfcce300.dll!#644` |
| [Adobe Reader](https://archive.org/details/tucows_360190_Adobe_Reader) | `acrobatreader-ppc2002.exe` | Does not start | This is a desktop Windows program, not a Pocket PC one. |
| [Adobe Reader](https://archive.org/details/tucows_360190_Adobe_Reader) | `acrobatreader-ppcarm.exe` | Does not start | This is a desktop Windows program, not a Pocket PC one. |
| [Aeon of Discovery](https://archive.org/details/tucows_371843_Aeon_of_Discovery) | `AODsetupTrial.exe` | Does not start | The game needs a file that is not in the package. |
| [Aero Map V 2.40 ARM](https://archive.org/details/aero-map-v-2.40-arm) | `AeroMap v2.40 ARM.zip` | Does not start | The game needs a file that comes with the Pocket PC itself. Missing: `mfcce300.dll!#644` |
| [AirTunes!](https://archive.org/details/tucows_261644_AirTunes) | `airtunes2.zip` | Does not start |  |
| [Android on windows mobile htc device](https://archive.org/details/frx-06-full-bundle-25.3.2011) | `android/CLRCAD.exe` | Does not start |  |
| [Android on windows mobile htc device](https://archive.org/details/frx-06-full-bundle-25.3.2011) | `android/haret.exe` | Does not start | The game uses an instruction PocketPC3630 does not handle yet. |
| [Applian CoolCalc (Extracted)](https://archive.org/details/tucows_152073_CoolCalc_extracted) | `CoolCalc/CoolCalc.arm.CAB` | Does not start | The game stopped on something the emulator cannot do yet. |
| [AstroBlaster](https://archive.org/details/tucows_226736_AstroBlaster) | `AstroBlasterDemo.exe` | Does not start | This is a desktop Windows program, not a Pocket PC one. |
| [AstroWin Expert Edition](https://archive.org/details/tucows_343848_AstroWin_Expert_Edition) | `AstroWinCESetup.exe` | Does not start | This is a desktop Windows program, not a Pocket PC one. |
| [AstroWinCE Quick Edition](https://archive.org/details/tucows_343847_AstroWinCE_Quick_Edition) | `AstroWinCESetup.exe` | Does not start | This is a desktop Windows program, not a Pocket PC one. |
| [AstroWinCE Standard Edition](https://archive.org/details/tucows_343163_AstroWinCE_Standard_Edition) | `AstroWinCESetup.exe` | Does not start | This is a desktop Windows program, not a Pocket PC one. |
| [ATAlarm for PPC2002](https://archive.org/details/tucows_341587_ATAlarm_for_PPC2002) | `ATAlarm2002_Setup.exe` | Does not start |  |
| [AuctionTracker Wireless](https://archive.org/details/tucows_182904_AuctionTracker_Wireless) | `auctiontrackersetupdemo.exe` | Does not start | The game needs a file that comes with the Pocket PC itself. Missing: `mfcce300.dll!#644` |
| [Back Up and Strip](https://archive.org/details/tucows_217797_Back_Up_and_Strip) | `bupsdemo.zip` | Does not start | This is a desktop Windows program, not a Pocket PC one. |
| [BCL VoiceMP3](https://archive.org/details/tucows_291109_BCL_VoiceMP3) | `voicemp3demo.exe` | Does not start | The game needs a file that comes with the Pocket PC itself. Missing: `mfcce300.dll!#644` |
| [Belkin Thumb Keyboard for Dell Axim X50/X51 Handhelds Drivers](https://archive.org/details/f8d0502) | `f8d0502/thumbboard.MS.PocketPC.CAB` | Does not start | The game needs a file that comes with the Pocket PC itself. Missing: `mfcce300.dll!#644` |
| [Belkin Thumb Keyboard for Dell Axim X50/X51 Handhelds Drivers](https://archive.org/details/f8d0502) | `Original Disc Files/thumbboard.MS.PocketPC.CAB` | Does not start | The game needs a file that comes with the Pocket PC itself. Missing: `mfcce300.dll!#644` |
| [BijiWriter](https://archive.org/details/tucows_159829_BijiWriter) | `bijiwriter-hpc-sh3.zip` | Does not start | The game stopped on something the emulator cannot do yet. Missing: `COREDLL.dll!GetCaretPos` |
| [Bingo Island](https://archive.org/details/bingo-island) | `Bingo Island.zip` | Does not start | The game needs a file that is not in the package. |
| [Blueprint Software NetTools](https://archive.org/details/tucows_294919_Blueprint_Software_NetTools) | `RSC_MT_05.zip` | Does not start | The game needs a file that comes with the Pocket PC itself. Missing: `mfcce300.dll!#644` |
| [Brain Stadium](https://archive.org/details/tucows_240096_Brain_Stadium) | `BrainStadiumEnglish100.ZIP` | Does not start | The game needs a file that comes with the Pocket PC itself. Missing: `mfcce300.dll!#644` |
| [Bunnydoku](https://archive.org/details/tucows_333232_Bunnydoku) | `BunnydokuSetup-Trial.exe` | Does not start | This is a .NET Compact Framework program, which PocketPC3630 cannot run. |
| [Call of Duty 2 for Windows Mobile](https://archive.org/details/Call_of_Duty_2-for-Windows-Mobile) | `CallOfDuty2_xScale.rar` | Does not start |  |
| [Call of Duty 2 for Windows Mobile](https://archive.org/details/Call_of_Duty_2-for-Windows-Mobile) | `cod2_goforce.zip` | Does not start |  |
| [Cashman](https://archive.org/details/tucows_32397_Cashman) | `cashmanSH3.exe` | Does not start | The game needs a file that comes with the Pocket PC itself. Missing: `mfcce20.dll!#560` |
| [CCExplore](https://archive.org/details/tucows_32472_CCExplore) | `cd003-0a.zip` | Does not start | The game needs a file that comes with the Pocket PC itself. Missing: `mfcce211.dll!#631` |
| [CCNetTools](https://archive.org/details/tucows_50387_CCNetTools) | `ccnet-tools.zip` | Does not start | The game needs a file that comes with the Pocket PC itself. Missing: `mfcce211.dll!#631` |
| [CCRegEdit](https://archive.org/details/tucows_32473_CCRegEdit) | `cd008-0a.zip` | Does not start | The game needs a file that comes with the Pocket PC itself. Missing: `mfcce211.dll!#631` |
| [CE Task Manager](https://archive.org/details/tucows_59033_CE_Task_Manager) | `cetaskmanager.exe` | Does not start | The game needs a file that is not in the package. Missing: `COREDLL.dll!GetCaretPos` |
| [CeCAD](https://archive.org/details/tucows_182640_CeCAD) | `CeCAD.exe` | Does not start | This is a desktop Windows program, not a Pocket PC one. |
| [CEdit](https://archive.org/details/tucows_368959_CEdit) | `CEdit_ARM_PPC_2003.zip` | Does not start | The game needs a file that comes with the Pocket PC itself. Missing: `mfcce300.dll!#644` |
| [Centro de Dispositivos de Windows Mobile](https://archive.org/details/wmdc_for_Win10_and_oward) | `WMDC.zip` | Does not start | This is a desktop Windows program, not a Pocket PC one. |
| [CETuner](https://archive.org/details/tucows_157681_CETuner) | `CETunerPPC.exe` | Does not start | This is a desktop Windows program, not a Pocket PC one. |
| [Co Pilot Live Pocket](https://archive.org/details/co-pilot-live-pocket.arm-cracked) | `CoPilot_Live Pocket.arm CRACKED.zip` | Does not start | The game needs a file that is not in the package. |
| [CodeStore](https://archive.org/details/tucows_335804_CodeStore) | `codestore.zip` | Does not start | This is a .NET Compact Framework program, which PocketPC3630 cannot run. |
| [Color Picker](https://archive.org/details/tucows_151970_Color_Picker) | `ColorPicker1.0.beta1.exe` | Does not start | This is a desktop Windows program, not a Pocket PC one. |
| [Comet VideoPhone](https://archive.org/details/tucows_345047_Comet_VideoPhone) | `videophonesetup.zip` | Does not start | This is a desktop Windows program, not a Pocket PC one. |
| [Cornered](https://archive.org/details/cornered_202505) | `Cornered.zip` | Does not start | The game stopped on something the emulator cannot do yet. |
| [CrunchIt](https://archive.org/details/tucows_293751_CrunchIt) | `CrunchIt410_TRY.zip` | Does not start | The game needs a file that comes with the Pocket PC itself. Missing: `mfcce300.dll!#644` |
| [CryptoStorage](https://archive.org/details/tucows_361308_CryptoStorage) | `crstorppc.exe` | Does not start | The game needs a file that is not in the package. Missing: `COREDLL.dll!RegDeleteKeyW`, `COREDLL.dll!ActivateDevice` |
| [CShekhar WM 6.1.2a R16.P32Public](https://archive.org/details/cshekharwm6.1.2ar16.p32) | `C_ShekharWM6.1.2aR16.P32_public.rar` | Does not start | The game needs a file that is not in the package. |
| [Cyber Mod](https://archive.org/details/tucows_296603_Cyber_Mod) | `CyberMod_Install.exe` | Does not start | The game needs a file that comes with the Pocket PC itself. Missing: `mfcce300.dll!#644` |
| [Data Bridge](https://archive.org/details/tucows_32351_Data_Bridge) | `dbridge1110.exe` | Does not start | The game needs a file that comes with the Pocket PC itself. Missing: `mfcce20.dll!#560` |
| [DB Anywhere](https://archive.org/details/tucows_209743_DB_Anywhere) | `DBAnywhereSetup_t.exe` | Does not start | The game needs a file that is not in the package. Missing: `mfcce300.dll!#644` |
| [DB Delete](https://archive.org/details/tucows_32338_DB_Delete) | `dbdelMIPSM.zip` | Does not start | The game stopped on something the emulator cannot do yet. Missing: `mfcwce.dll!#458` |
| [DBExplorer](https://archive.org/details/tucows_32339_DBExplorer) | `DBExplorersh3.exe` | Does not start | The game needs a file that comes with the Pocket PC itself. Missing: `mfcce20.dll!#560` |
| [Dell Axim X5 ROM Update for Pocket PC 2002, v.03050300EN, A00 Axim X5 ROM Update for Pocket PC 2002, Advanced and Basic model](https://archive.org/details/r-58550) | `R58550.EXE` | Does not start | The game stopped on something the emulator cannot do yet. |
| [DEXplor](https://archive.org/details/tucows_267105_DEXplor) | `DEXplor.exe` | Does not start | The game needs a file that comes with the Pocket PC itself. Missing: `mfcce300.dll!#644` |
| [Dialect Standard](https://archive.org/details/tucows_32353_Dialect_Standard) | `Dialect_Std_Sh3_200.zip` | Does not start | The game needs a file that is not in the package. Missing: `mfcce20.dll!#560` |
| [dNote](https://archive.org/details/tucows_183443_dNote) | `dNotePPC.exe` | Does not start | The game needs a file that is not in the package. Missing: `cbccce.dll!??0CString@@QAA@XZ`, `cbccce.dll!?OpenKey@@YA_NAAPAUHKEY__@@PBG@Z`, `cbccce.dll!code4initLow` |
| [dNote](https://archive.org/details/tucows_32574_dNote) | `dNote.exe` | Does not start | The game needs a file that is not in the package. Missing: `cbccce.dll!??0CString@@QAA@XZ`, `cbccce.dll!?OpenKey@@YA_NAAPAXPBG@Z`, `cbccce.dll!code4initLow` |
| [Dollars and CEnts](https://archive.org/details/tucows_32402_Dollars_and_CEnts) | `dandchh.zip` | Does not start | The game needs a file that comes with the Pocket PC itself. Missing: `mfcce20.dll!#516` |
| [Don't Forget Dates](https://archive.org/details/tucows_221639_Don_t_Forget_Dates) | `dontforgetdates.zip` | Does not start |  |
| [dotPocket](https://archive.org/details/tucows_332728_dotPocket) | `dotpocket.exe` | Does not start | The game needs a file that comes with the Pocket PC itself. Missing: `mfcce300.dll!#644` |
| [Double Launcher](https://archive.org/details/tucows_280206_Double_Launcher) | `DoubleLauncher200Setup.exe` | Does not start | This is a desktop Windows program, not a Pocket PC one. |
| [Drivers for Nexian NXC3100 NexiCam Digital Camera sleeve for HP iPaq Pocket PC](https://archive.org/details/nexicam-ipaq-drivers) | `NexiCam.2577.cab` | Does not start | The game needs a file that comes with the Pocket PC itself. Missing: `mfcce300.dll!#644` |
| [Dyna Metric](https://archive.org/details/tucows_32303_Dyna_Metric) | `dmetric1310.exe` | Does not start | The game needs a file that is not in the package. Missing: `COREDLL.dll!sh__setjmp` |
| [E-forms](https://archive.org/details/tucows_77147_E-forms) | `e-forms.exe` | Does not start | This is a desktop Windows program, not a Pocket PC one. |
| [e-pill PillPAL Medication Reminder](https://archive.org/details/tucows_300643_e-pill_PillPAL_Medication_Reminder) | `freepillpal.zip` | Does not start | The game needs a file that comes with the Pocket PC itself. Missing: `mfcce300.dll!#644` |
| [English-Hungarian and Hungarian-English dictionary](https://archive.org/details/tucows_253681_English-Hungarian_and_Hungarian-English_dictionary) | `Hun_SlovoEd_lite.zip` | Does not start | This is a desktop Windows program, not a Pocket PC one. |
| [EuroCalculator WinCE (Extracted)](https://archive.org/details/EuroCalculator-1999-06-27-ce-hpcpro) | `EuroCalculator.ARM.CAB` | Does not start | The game needs a file that comes with the Pocket PC itself. Missing: `mfcce211.dll!#612` |
| [EuroCalculator WinCE (Extracted)](https://archive.org/details/EuroCalculator-1999-06-27-ce-hpcpro) | `GetSetGo!!!.CAB` | Does not start | The game needs a file that comes with the Pocket PC itself. Missing: `mfcce211.dll!#612` |
| [Events](https://archive.org/details/tucows_203075_Events) | `EventsARM.exe` | Does not start | The game needs a file that comes with the Pocket PC itself. Missing: `mfcce300.dll!#644` |
| [eWallet](https://archive.org/details/tucows_222152_eWallet) | `eWalletPEK30.exe` | Does not start | Missing: `COREDLL.dll!RegisterClipboardFormatW`, `COREDLL.dll!SHGetShortcutTarget` |
| [Exclusive Gameloft games for Samsung Omnia i900 (WiTu)](https://archive.org/details/omnia-loft) | `Asphalt4.exe` | Does not start |  |
| [Exclusive Gameloft games for Samsung Omnia i900 (WiTu)](https://archive.org/details/omnia-loft) | `Prince_Of_Persia_HD__Omnia_.cab` | Does not start | The game stopped on something the emulator cannot do yet. |
| [Facebook for Windows Mobile 6.x](https://archive.org/details/facebook-windows-mobile-6-x) | `Facebook_v1.0.0.7.CAB` | Does not start | The game needs a file that is not in the package. Missing: `COREDLL.dll!LoadLibraryExW`, `HTMLVIEW.DLL!#1`, `COREDLL.dll!CompareStringW` |
| [Facebook for Windows Mobile 6.x](https://archive.org/details/facebook-windows-mobile-6-x) | `Facebook_v1.0.0.7p.CAB` | Does not start | The game needs a file that is not in the package. Missing: `COREDLL.dll!LoadLibraryExW`, `HTMLVIEW.DLL!#1`, `COREDLL.dll!CompareStringW` |
| [Ferrari GT: Evolution - HTC HD2/Toshiba Tsunagi Port (Android/Windows)](https://archive.org/details/ferrari_gt_port_aw) | `FerrariGT_Win.zip` | Does not start |  |
| [Ferrari GT: Evolution HD+ for HTC HD2 and Toshiba Tsunagi](https://archive.org/details/ferrari-gt-hd2-wm) | `FGTH_FerrariGT.exe` | Does not start |  |
| [FFtp](https://archive.org/details/tucows_32192_FFtp) | `ffp018.zip` | Does not start |  |
| [FingerCE](https://archive.org/details/tucows_32193_FingerCE) | `fingerCE.zip` | Does not start | Missing: `COREDLL.dll!FindResource` |
| [Firefox Windows Mobile 10](https://archive.org/details/fennec-1.1a1.en-US.wince-arm) | `fennec-1.1a1.en-US.wince-arm.exe` | Does not start | The game needs a file that is not in the package. |
| [Firefox Windows Mobile 10](https://archive.org/details/fennec-1.1a1.en-US.wince-arm) | `fennec-1.1a1.en-US.wince-arm.zip` | Does not start | Missing: `mozce_shunt.dll!malloc` |
| [First Diagramming](https://archive.org/details/tucows_181965_First_Diagramming) | `FirstSetup.exe` | Does not start | The game needs a file that comes with the Pocket PC itself. Missing: `mfcce300.dll!#644` |
| [Flash Toolset Professional Edition](https://archive.org/details/tucows_328895_Flash_Toolset_Professional_Edition) | `SetupFlashToolset2ProfessionalTrial.exe` | Does not start | This is a desktop Windows program, not a Pocket PC one. |
| [Fograin ARM Full](https://archive.org/details/fograinarmfull) | `Fograin_ARM_full.exe` | Does not start | This is a desktop Windows program, not a Pocket PC one. |
| [Fonix Commander](https://archive.org/details/tucows_240266_Fonix_Commander) | `VoiceCentralLite.exe` | Does not start | This is a desktop Windows program, not a Pocket PC one. |
| [Fonix TimeTalk](https://archive.org/details/tucows_240262_Fonix_TimeTalk) | `timetalkppc.exe` | Does not start | This is a desktop Windows program, not a Pocket PC one. |
| [Formulae 1](https://archive.org/details/tucows_32308_Formulae_1) | `F1WinCEdemoMIPS.exe` | Does not start | The game stopped on something the emulator cannot do yet. |
| [Forty Thieves](https://archive.org/details/tucows_300532_Forty_Thieves) | `FortyThievesPPC.exe` | Does not start | The game needs a file that comes with the Pocket PC itself. Missing: `mfcce300.dll!#644` |
| [Four Winds Mah Jong](https://archive.org/details/tucows_327251_Four_Winds_Mah_Jong) | `4WPPC2002Demo.exe` | Does not start | This is a desktop Windows program, not a Pocket PC one. |
| [Freda: the free ebook reader for Windows Mobile 6+ and Windows CE](https://archive.org/details/freda-cab-2-0-104-wm-6) | `FredaCab_2_0_104_WinCE.CAB` | Does not start | This is a .NET Compact Framework program, which PocketPC3630 cannot run. |
| [Freda: the free ebook reader for Windows Mobile 6+ and Windows CE](https://archive.org/details/freda-cab-2-0-104-wm-6) | `FredaCab_2_0_104_WM6.CAB` | Does not start | This is a .NET Compact Framework program, which PocketPC3630 cannot run. |
| [Fruit Machine](https://archive.org/details/tucows_226733_Fruit_Machine) | `FruityDemo.exe` | Does not start | This is a desktop Windows program, not a Pocket PC one. |
| [FTNet](https://archive.org/details/tucows_32194_FTNet) | `ftn035.zip` | Does not start |  |
| [GameBag One](https://archive.org/details/tucows_222190_GameBag_One) | `GameBagOnePKT30Day.exe` | Does not start | The game stopped on something the emulator cannot do yet. |
| [GameBag One](https://archive.org/details/tucows_32133_GameBag_One) | `GameBagOneHPC30Day.exe` | Does not start | The game stopped on something the emulator cannot do yet. |
| [Gangsta Race](https://archive.org/details/gangsta-race) | `GangstaRace.zip` | Does not start | The game stopped on something the emulator cannot do yet. |
| [Gen Plusplus Pocket Pc Genesis Emulator (GX.DDL Included :D)](https://archive.org/details/gen-pp) | `GenPP.exe` | Does not start | The game needs a file that is not in the package. |
| [GetPDAScreen](https://archive.org/details/get-pdascreen) | `GetPDAScreen.zip` | Does not start | The game needs a file that comes with the Pocket PC itself. Missing: `mfcce300.dll!#644` |
| [GetSetGo](https://archive.org/details/tucows_32500_GetSetGo) | `getsetgo1210.exe` | Does not start | The game stopped on something the emulator cannot do yet. Missing: `mfcce201.dll!#649` |
| [Google Maps for Windows Mobile (Actually Works)](https://archive.org/details/google-maps.ppc) | `GoogleMaps.ppc.CAB` | Does not start | The game needs a file that is not in the package. Missing: `COREDLL.dll!PowerPolicyNotify` |
| [Google Maps for Windows Mobile (PDA)](https://archive.org/details/google-maps-windows-mobile) | `GoogleMaps.CAB` | Does not start | The game needs a file that is not in the package. Missing: `COREDLL.dll!SipEnumIM` |
| [Google Maps Mobile (v3.0.1.6) (Windows Mobile Port)](https://archive.org/details/google-maps-mobile-v-3.0.1.6) | `Google Maps Mobile (v3.0.1.6).CAB` | Does not start | The game needs a file that is not in the package. Missing: `COREDLL.dll!PowerPolicyNotify` |
| [GoogleNavigator for Windows Mobile](https://archive.org/details/googlenavWM) | `NETCFv2.wm.armv4i.cab` | Does not start | This is a .NET Compact Framework program, which PocketPC3630 cannot run. |
| [GoogleNavigator for Windows Mobile](https://archive.org/details/googlenavWM) | `GlobalNavigator.cab` | Does not start | This is a .NET Compact Framework program, which PocketPC3630 cannot run. |
| [GOST Scheduler](https://archive.org/details/tucows_277177_GOST_Scheduler) | `timetable.zip` | Does not start |  |
| [Graphic-Plotter](https://archive.org/details/Graphic-Plotter-1999-04-13-ce-hpc) | `Plot.hpc_mips.CAB` | Does not start | The game stopped on something the emulator cannot do yet. Missing: `COREDLL.dll!GetCaretPos` |
| [Graphic-Plotter](https://archive.org/details/Graphic-Plotter-1999-04-13-ce-hpc) | `Plot.hpc_sh3.CAB` | Does not start | The game stopped on something the emulator cannot do yet. Missing: `COREDLL.dll!GetCaretPos` |
| [GraphIt](https://archive.org/details/tucows_32312_GraphIt) | `GraphIt_HPC_2_2s.zip` | Does not start | The game stopped on something the emulator cannot do yet. Missing: `COREDLL.dll!GetCaretPos` |
| [Griff Music Sequencer for PocketPC](https://archive.org/details/griff-setup) | `griff.arm.CAB` | Does not start | The game needs a file that comes with the Pocket PC itself. Missing: `mfcce300.dll!#644` |
| [Guitar Hero Mobile 3 V 1.1.0](https://archive.org/details/guitar-hero-mobile-3-v-1.1.0) | `Guitar Hero Mobile 3 v1.1.0.zip` | Does not start |  |
| [Haali Reader 2.0b227 - a book reader for Windows CE](https://archive.org/details/haali-reader-2.0b227-a-book-reader-for-windows-ce) | `CE.NET/HaaliReader.NET.ARMV4.zip` | Does not start | The game needs a file that comes with the Pocket PC itself. Missing: `mfcce400.dll!#648` |
| [Haali Reader 2.0b264 for Pocket PC - a book reader for Windows CE](https://archive.org/details/20221104_20221104_0126) | `POCKET PC/HaaliReader.PPC.ARM.zip` | Does not start | The game needs a file that comes with the Pocket PC itself. Missing: `mfcce300.dll!#644` |
| [Hand O' Crazy 8s](https://archive.org/details/tucows_253871_Hand_O_Crazy_8s) | `HandOCrazy8sInst10.zip` | Does not start | The game stopped on something the emulator cannot do yet. |
| [Handheld Handler](https://archive.org/details/tucows_237281_Handheld_Handler) | `handheldhandlertrial.exe` | Does not start | The game needs a file that comes with the Pocket PC itself. Missing: `mfcce300.dll!#644` |
| [Handybook](https://archive.org/details/tucows_32501_Handybook) | `handybookhpcsh3.zip` | Timed out | Did not finish starting within 120 seconds |
| [HeroCraft Misc Palm/Windows Mobile Titles](https://archive.org/details/install-robo-wm) | `ArcadesForAdultsWMSetup_240x240.exe` | Does not start | The game uses an instruction PocketPC3630 does not handle yet. |
| [HP iPAQ h1900 series (h1940) ROM Update 06 Apr 2004](https://archive.org/details/ipaq-h1900-series-h1940-rom-update-06-apr-2004) | `Spanish (1.10)/sp26262.exe` | Does not start | This is a desktop Windows program, not a Pocket PC one. |
| [HP iPAQ h1900 series (h1940) ROM Update 06 Apr 2004](https://archive.org/details/ipaq-h1900-series-h1940-rom-update-06-apr-2004) | `French (1.10)/sp26260.exe` | Does not start | This is a desktop Windows program, not a Pocket PC one. |
| [HP iPAQ h4350/h4355 Pocket PC ROM Update 1.10](https://archive.org/details/sp-28375) | `SP28375.exe` | Does not start | This is a desktop Windows program, not a Pocket PC one. |
| [HP iPAQ rx3700 ROM Update 1.01.11](https://archive.org/details/sp29773) | `sp29773.exe` | Does not start | This is a desktop Windows program, not a Pocket PC one. |
| [HPC dbExplorer](https://archive.org/details/tucows_219086_HPC_dbExplorer) | `hpcdbex.exe` | Does not start | This is a desktop Windows program, not a Pocket PC one. |
| [HPC dbExplorer](https://archive.org/details/tucows_32343_HPC_dbExplorer) | `hpcdbex.exe` | Does not start | This is a desktop Windows program, not a Pocket PC one. |
| [HPC NetProfile](https://archive.org/details/tucows_220181_HPC_NetProfile) | `hpcnetprofile.exe` | Does not start | This is a desktop Windows program, not a Pocket PC one. |
| [HPC NetProfile](https://archive.org/details/tucows_32196_HPC_NetProfile) | `hpcnetprofile.exe` | Does not start | This is a desktop Windows program, not a Pocket PC one. |
| [HPC Notes](https://archive.org/details/tucows_32576_HPC_Notes) | `hpcnotes304std.exe` | Does not start | This is a desktop Windows program, not a Pocket PC one. |
| [HPC Notes Standard Edition](https://archive.org/details/tucows_219690_HPC_Notes_Standard_Edition) | `hpcnotes307std.exe` | Does not start | This is a desktop Windows program, not a Pocket PC one. |
| [HPC Spell](https://archive.org/details/tucows_219652_HPC_Spell) | `hpcspell26.exe` | Does not start | This is a desktop Windows program, not a Pocket PC one. |
| [HPC Spell](https://archive.org/details/tucows_32278_HPC_Spell) | `hpcspell25.exe` | Does not start | This is a desktop Windows program, not a Pocket PC one. |
| [HPC Translate](https://archive.org/details/tucows_32279_HPC_Translate) | `hpctrans.exe` | Does not start | This is a desktop Windows program, not a Pocket PC one. |
| [HTC MAX 4G firmware and files collection](https://archive.org/details/htc-max-4g-collection) | `HSPL and SSPL/Quar_SSPL_Manual.zip` | Does not start | The game stopped on something the emulator cannot do yet. |
| [HTC S620/HTC Excalibur/HTC EXCA100/200 - ROM/Misc. archive](https://archive.org/details/HTC_S620_Archive) | `T-Mobile_DASH_WM5_V_1.11.531.1.exe` | Does not start | This is a desktop Windows program, not a Pocket PC one. |
| [HTC S620/HTC Excalibur/HTC EXCA100/200 - ROM/Misc. archive](https://archive.org/details/HTC_S620_Archive) | `RUU_Excalibur_HTC_WWE_Brightpoint_US_1.33.422.1_4.1.13.51_03.07.90_Ship.exe` | Does not start | This is a desktop Windows program, not a Pocket PC one. |
| [htc_charmer_noid](https://archive.org/details/htccharmernoid) | `HTC_Charmer_noid.zip` | Does not start |  |
| [Hum MP3 Player](https://archive.org/details/tucows_32280_Hum_MP3_Player) | `Hum1_6e.zip` | Does not start | The game needs a file that is not in the package. Missing: `COREDLL.dll!CreatePenIndirect` |
| [Hybrid Rasteroid 3.1](https://archive.org/details/hybrid-rasteroid-3.1) | `rasteroid3.1.zip` | Does not start | Missing: `libEGL.dll!eglBindAPI`, `libEGL.dll!eglGetDisplay`, `libEGL.dll!eglGetError` |
| [IA Presenter](https://archive.org/details/tucows_200345_IA_Presenter) | `IAPresenter210EN.zip` | Does not start | This is a desktop Windows program, not a Pocket PC one. |
| [IA ScreenShot](https://archive.org/details/tucows_196426_IA_ScreenShot) | `IAScreenShot20EN.exe` | Does not start | This is a desktop Windows program, not a Pocket PC one. |
| [iCal Parser for Pocket PCs](https://archive.org/details/ical-parser-ppc) | `CalParseAppInstaller2.1.1.CAB` | Does not start | This is a .NET Compact Framework program, which PocketPC3630 cannot run. |
| [Insaniquarium for Pocket PC](https://archive.org/details/insaniquarium-ppc-v1-2) | `insaniquarium-ppc-v1-2.cab` | Does not start |  |
| [Intelli Chess](https://archive.org/details/tucows_306976_Intelli_Chess) | `ppcic.exe` | Does not start | The game needs a file that comes with the Pocket PC itself. Missing: `mfcce300.dll!#644` |
| [iPAQ Image Zone](https://archive.org/details/i-paq-image-zone) | `iPAQ Image Zone.zip` | Does not start | The game stopped on something the emulator cannot do yet. Missing: `AYGSHELL.dll!#114` |
| [iSignIt](https://archive.org/details/tucows_254321_iSignIt) | `iSignItARM.exe` | Does not start | This is a desktop Windows program, not a Pocket PC one. |
| [iStopWatch](https://archive.org/details/tucows_214016_iStopWatch) | `iStopWatch.exe` | Does not start | The game needs a file that comes with the Pocket PC itself. Missing: `mfcce300.dll!#644` |
| [J9 JVM MIDP 2.0 For Windows Mobile 2003 (ARM)](https://archive.org/details/j9-midp20-wm2003-arm_221) | `j9-midp20-wm2003-arm_22.zip` | Does not start | The game stopped on something the emulator cannot do yet. Missing: `J9VMALL22.dll!j9port_create_library`, `J9VMALL22.dll!j9port_startup_library` |
| [Kara-O-Pocket](https://archive.org/details/tucows_359702_Kara-O-Pocket) | `karaopocket-211-unregistered-setup.exe` | Does not start | The game stopped on something the emulator cannot do yet. Missing: `GD104b.dll!??0CGapiSurface@@QAA@XZ`, `GD104b.dll!?SetColorKey@CGapiSurface@@QAAJK@Z`, `GD104b.dll!?CreateSurface@CGapiSurface@@QAAJPAEK@Z` |
| [Kilmist Registry Editor](https://archive.org/details/tucows_274481_Kilmist_Registry_Editor) | `KilmistRegEdit.exe` | Does not start | This is a desktop Windows program, not a Pocket PC one. |
| [KingKanji](https://archive.org/details/tucows_32287_KingKanji) | `kkhpc20_jpn.exe` | Does not start | The game stopped on something the emulator cannot do yet. |
| [KJV Bible Reader](https://archive.org/details/tucows_32288_KJV_Bible_Reader) | `bible_211hand_mips.zip` | Does not start | The game needs a file that is not in the package. |
| [ListMan](https://archive.org/details/tucows_208716_ListMan) | `ListMan.zip` | Does not start | The game needs a file that comes with the Pocket PC itself. Missing: `mfcce300.dll!#644` |
| [ListPro](https://archive.org/details/tucows_222110_ListPro) | `ListProPKT30Day.exe` | Does not start | Missing: `COREDLL.dll!RegisterClipboardFormatW`, `COREDLL.dll!SHGetShortcutTarget`, `AYGSHELL.dll!#69` |
| [LogicStar](https://archive.org/details/tucows_216836_LogicStar) | `LogicStar.zip` | Does not start | The game needs a file that comes with the Pocket PC itself. Missing: `mfcce300.dll!#644` |
| [lokiwiz](https://archive.org/details/lokiwiz) | `lokiwiz02.zip` | Does not start |  |
| [LowFat Invoice](https://archive.org/details/tucows_289830_LowFat_Invoice) | `lowfatinvoice.exe` | Does not start | This is a desktop Windows program, not a Pocket PC one. |
| [M3 Mobile OX10 drivers (CE and WM)](https://archive.org/details/M3_OX10) | `M3OX10CE_FlashDisk.zip` | Does not start | The game stopped on something the emulator cannot do yet. |
| [MakeFaces](https://archive.org/details/tucows_366892_MakeFaces) | `makefaces_ppc_wag.zip` | Does not start | The game needs a file that comes with the Pocket PC itself. Missing: `mfcce300.dll!#644` |
| [MAPinPocket](https://archive.org/details/tucows_51457_MAPinPocket) | `maps1210.exe` | Does not start | The game needs a file that is not in the package. Missing: `mfcce201.dll!#649` |
| [Marble Alley](https://archive.org/details/tucows_226737_Marble_Alley) | `MarbleAlleyDemo.exe` | Does not start | This is a desktop Windows program, not a Pocket PC one. |
| [Maze Master CE](https://archive.org/details/tucows_32092_Maze_Master_CE) | `Maze.zip` | Does not start | The game stopped on something the emulator cannot do yet. |
| [Maze Wrangler](https://archive.org/details/tucows_52386_Maze_Wrangler) | `MazeWranglerMIPS.zip` | Does not start | The game stopped on something the emulator cannot do yet. Missing: `COREDLL.dll!GetCaretPos` |
| [Mazera](https://archive.org/details/Mazera) | `Mazera.zip` | Does not start |  |
| [MC70 Clean boot package](https://archive.org/details/MC70-reset) | `70XXCLEANBOOT.zip` | Does not start | Missing: `AYGSHELL.dll!#2011` |
| [MC70 Clean boot package](https://archive.org/details/MC70-reset) | `70XXCLEANBOOTANDBLANK.zip` | Does not start | Missing: `AYGSHELL.dll!#2011` |
| [Mega Touch 2 Hard SPL Kit By Jocky 2001](https://archive.org/details/megatouch2hardsplkitbyjocky2001) | `Custom_Rom_Flasher.zip` | Does not start |  |
| [Micaceler Pocket Ludo](https://archive.org/details/tucows_245994_Micaceler_Pocket_Ludo) | `PLSetup.exe` | Does not start | This is a desktop Windows program, not a Pocket PC one. |
| [Micaceler Pocket XWord](https://archive.org/details/tucows_325034_Micaceler_Pocket_XWord) | `PXSetup.exe` | Does not start | This is a desktop Windows program, not a Pocket PC one. |
| [Microsoft Device Emulator 3.0](https://archive.org/details/msdevice-emu) | `MSDeviceEmu.zip` | Does not start | This is a desktop Windows program, not a Pocket PC one. |
| [Microsoft Device Emulator builds](https://archive.org/details/microsoft_emulators) | `Microsoft Device Emulator 2.0 - Standalone Release/vs_emulator-X64Addon.exe` | Does not start | This is a desktop Windows program, not a Pocket PC one. |
| [Microsoft Device Emulator builds](https://archive.org/details/microsoft_emulators) | `Microsoft Device Emulator 2.0 - Standalone Release/vs_emulator.exe` | Does not start | This is a desktop Windows program, not a Pocket PC one. |
| [Microsoft Device Emulator for Windows Mobile 5](https://archive.org/details/MS_DeviceEmulator) | `DeviceEmulator-1.zip` | Does not start | This is a desktop Windows program, not a Pocket PC one. |
| [Microsoft Entertainment Pocket Pak Anoter Pak](https://archive.org/details/microsoft-entertainment-pocket-pak-anoter-pak) | `Microsoft Entertainment PocketPak anoter pak.zip` | Does not start |  |
| [Microsoft Office Mobile (v6.1) (Windows Mobile Port)](https://archive.org/details/office2007_202610) | `office2007.CAB` | Does not start | The game needs a file that is not in the package. Missing: `COREDLL.dll!LoadLibraryExW`, `AYGSHELL.dll!#310`, `OFFICE.dll!RMLoadLibrary` |
| [Microsoft Office Mobile 6.1: Upgrade for Microsoft Office 2007 file formats ARM](https://archive.org/details/officemobile_licensed_0409_us_arm) | `Microsoft Office Mobile/Office_wpc_WM5.CAB` | Does not start | The game needs a file that is not in the package. Missing: `COREDLL.dll!LoadLibraryExW`, `AYGSHELL.dll!#310`, `OFFICE.dll!RMLoadLibrary` |
| [Microsoft Office Mobile 6.1: Upgrade for Microsoft Office 2007 file formats ARM](https://archive.org/details/officemobile_licensed_0409_us_arm) | `Microsoft Office Mobile/office_wpc_WM6.CAB` | Does not start | The game needs a file that is not in the package. Missing: `COREDLL.dll!__CxxThrowException`, `COREDLL.dll!LoadLibraryExW`, `AYGSHELL.dll!#310` |
| [Microsoft Pocket Expense Powertoy for Pocket PC](https://archive.org/details/microsoft-pocket-expense) | `PocketExpense.Arm 1100 (4K) v3.00.CAB` | Does not start |  |
| [Microsoft Pocket Expense Powertoy for Pocket PC](https://archive.org/details/microsoft-pocket-expense) | `PocketExpense.SH 3 (1K) v3.00.CAB` | Does not start | The game stopped on something the emulator cannot do yet. |
| [Microsoft Pocket PC 2002 Games Arcade Pocket Pak](https://archive.org/details/microsoft-pocket-pc-2002-games-arcade-pocket-pak) | `Microsoft Pocket PC 2002 Games Arcade Pocket Pak.exe` | Does not start | This is a desktop Windows program, not a Pocket PC one. |
| [Microsoft Pocket Streets 2002 RTM](https://archive.org/details/microsoft-pocket-streets-2002-rtm) | `P.RAP_ARM.CAB` | Does not start | Missing: `AYGSHELL.dll!#69`, `DocList.dll!DocList_Create` |
| [Microsoft PowerToys](https://archive.org/details/tucows_32635_Microsoft_PowerToys) | `ppctoys.exe` | Does not start | The game's program file cannot be loaded. |
| [Microsoft Reader for Windows Mobile (Pocket PC) 2.4.22.9229 (Extracted)](https://archive.org/details/msreader_arm_2.4.22.9229) | `msreader_arm.cab` | Does not start | The game needs a file that is not in the package. Missing: `EBookFx.dll!getEbakMetric`, `dmgr.dll!DRMInitialize`, `EBookFx.dll!initializeAppForEbookfx` |
| [Microsoft USB 2.0 Driver Fix for PocketPC Devices (for XP and 2000) - KB838989](https://archive.org/details/Microsofr-KB838989) | `R80052.EXE` | Does not start | The game's program file cannot be loaded. |
| [Microsoft Windows Mobile Device Center 6.1](https://archive.org/details/MicrosoftWindowsMobileDeviceCenter) | `drvupdate-amd64.exe` | Does not start | This is a desktop Windows program, not a Pocket PC one. |
| [MindSoft Pocket CleanUp](https://archive.org/details/tucows_313452_MindSoft_Pocket_CleanUp) | `mpcueng.zip` | Does not start |  |
| [Mixtikl PPC](https://archive.org/details/mixtikl-ppc) | `Mixtikl_PPC.zip` | Does not start | The game stopped on something the emulator cannot do yet. |
| [Mobile Channel Viewer](https://archive.org/details/tucows_32206_Mobile_Channel_Viewer) | `HPCChan.EXE` | Does not start | The game's program file cannot be loaded. |
| [Mobile Web Server 3.1 (Windows Mobile)](https://archive.org/details/mobilewebserver) | `MWS-PPC.CAB` | Does not start | The game needs a file that is not in the package. Missing: `COREDLL.dll!ConvertDefaultLocale`, `COREDLL.dll!CreatePatternBrush` |
| [Mobile Web Server 3.1 (Windows Mobile)](https://archive.org/details/mobilewebserver) | `MWS+PPC.CAB` | Does not start | The game needs a file that is not in the package. Missing: `COREDLL.dll!ConvertDefaultLocale`, `COREDLL.dll!CreatePatternBrush` |
| [MobiPassword](https://archive.org/details/tucows_291984_MobiPassword) | `mobipassword_ppc.exe` | Does not start | This is a desktop Windows program, not a Pocket PC one. |
| [Mocha VNC for Windows Mobile (v1.1) (Windows Mobile Port)](https://archive.org/details/smtvnc) | `smtvnc.cab` | Does not start | This is a .NET Compact Framework program, which PocketPC3630 cannot run. |
| [MxCalculator](https://archive.org/details/tucows_150376_MxCalculator) | `mxcalculator.zip` | Does not start | The game needs a file that comes with the Pocket PC itself. Missing: `mfcce300.dll!#644` |
| [My Contacts](https://archive.org/details/tucows_240400_My_Contacts) | `MyContactsTrial.zip` | Does not start | The game needs a file that comes with the Pocket PC itself. Missing: `mfcce300.dll!#644` |
| [My Last Cigarette](https://archive.org/details/tucows_228704_My_Last_Cigarette) | `Cigarette.zip` | Does not start |  |
| [MyScript](https://archive.org/details/tucows_184042_MyScript) | `MyScript_e110_Inter.exe` | Does not start | This is a desktop Windows program, not a Pocket PC one. |
| [MySports Soccer](https://archive.org/details/tucows_368298_MySports_Soccer) | `MSSoccer20.zip` | Does not start | This is a .NET Compact Framework program, which PocketPC3630 cannot run. |
| [Need for Speed: High Stakes for Pocket PC](https://archive.org/details/needforspeedhighstakesforpocketpc) | `1131BB194BB9F2EC36 (QVGA).cab` | Does not start | The game stopped on something the emulator cannot do yet. |
| [Need for Speed: High Stakes for Pocket PC](https://archive.org/details/needforspeedhighstakesforpocketpc) | `1733B8194BB9F2C851 (VGA).cab` | Does not start | The game stopped on something the emulator cannot do yet. |
| [NetFront Browser For Pocket PC / Windows Mobile](https://archive.org/details/netfront-browser-for-pocket-pc) | `NetFront_Browser_for_PocketPC.zip` | Does not start | The game needs a file that comes with the Pocket PC itself. Missing: `mfcce300.dll!#644` |
| [NetHack (v3.4.3) (Windows Mobile Port)](https://archive.org/details/nethack-343-ppc.ARM) | `nethack-343-ppc.ARM.CAB` | Does not start | The game stopped on something the emulator cannot do yet. Missing: `COREDLL.dll!CharNextW` |
| [NFS High Stakes for Pocket PC Port (Android/Windows)](https://archive.org/details/nfs-hs-wince-port) | `NFS_HighStakes_Win.zip` | Does not start | The game stopped on something the emulator cannot do yet. |
| [NFS Shift 3D and NFS Hot Pursuit 3D (Windows Mobile)](https://archive.org/details/nfss-hp-3d-wm) | `NFS.Shift.I900.Retail_Pantamorph.cab` | Does not start | The game stopped on something the emulator cannot do yet. Missing: `COREDLL.dll!?_set_new_handler@@YAP6AHI@ZP6AHI@Z@Z`, `COREDLL.dll!?_set_new_mode@@YAHH@Z` |
| [NFS Shift 3D and NFS Hot Pursuit 3D (Windows Mobile)](https://archive.org/details/nfss-hp-3d-wm) | `NFS2011.E101.Retail_Pantamorph.cab` | Does not start | The game stopped on something the emulator cannot do yet. Missing: `COREDLL.dll!?_set_new_handler@@YAP6AHI@ZP6AHI@Z@Z`, `COREDLL.dll!?_set_new_mode@@YAHH@Z` |
| [NFS Undercover HD (Windows Mobile)](https://archive.org/details/nfsuc-wm-hd) | `NFS_Undercover.cab` | Does not start | The game needs a file that is not in the package. |
| [Novii Remote Deluxe](https://archive.org/details/NRDeluxe) | `NRDeluxe.CAB` | Does not start | The game needs a file that comes with the Pocket PC itself. Missing: `mfcce300.dll!#644` |
| [obBASE](https://archive.org/details/tucows_32344_obBASE) | `obhpcmips.zip` | Does not start | The game stopped on something the emulator cannot do yet. Missing: `COREDLL.dll!GetCaretPos` |
| [OmniChex Web](https://archive.org/details/tucows_365573_OmniChex_Web) | `omnichex.zip` | Does not start | This is a .NET Compact Framework program, which PocketPC3630 cannot run. |
| [OpTrax](https://archive.org/details/tucows_32317_OpTrax) | `optrax.zip` | Does not start | The game needs a file that is not in the package. Missing: `COREDLL.dll!GetCaretPos` |
| [Other OEM Windows Phone ROM Archive](https://archive.org/details/wpdb_archive) | `Tools/HTC/ACDU.rar` | Does not start |  |
| [PADD](https://archive.org/details/tucows_168414_PADD) | `padd12.zip` | Timed out | Did not finish starting within 120 seconds |
| [Palbum Picture Optimizer](https://archive.org/details/tucows_256366_Palbum_Picture_Optimizer) | `palbumpo_setup.exe` | Does not start | The game needs a file that comes with the Pocket PC itself. Missing: `mfcce300.dll!#35`, `mfcce300.dll!#644` |
| [Palbum Picture Viewer](https://archive.org/details/tucows_256577_Palbum_Picture_Viewer) | `palbumpv_setup.exe` | Does not start | The game needs a file that comes with the Pocket PC itself. Missing: `mfcce300.dll!#644` |
| [Palm GB 101 Windows CE](https://archive.org/details/palm-gb-101-windows-ce) | `keypad.exe` | Does not start |  |
| [Patient Tracker](https://archive.org/details/tucows_70256_Patient_Tracker) | `ptce51.exe` | Does not start | This is a desktop Windows program, not a Pocket PC one. |
| [pdsCONTACTS](https://archive.org/details/tucows_32597_pdsCONTACTS) | `pdsContactss.zip` | Does not start | The game needs a file that is not in the package. Missing: `COREDLL.dll!GetCaretPos` |
| [PE Encrypt](https://archive.org/details/tucows_208768_PE_Encrypt) | `peencrypt50.zip` | Does not start | The game needs a file that comes with the Pocket PC itself. Missing: `mfcce300.dll!#644` |
| [PE Zip Explorer](https://archive.org/details/tucows_228729_PE_Zip_Explorer) | `pezip50.zip` | Does not start | The game needs a file that comes with the Pocket PC itself. Missing: `mfcce300.dll!#121`, `mfcce300.dll!#184`, `mfcce300.dll!#644` |
| [Penokio OutLynk](https://archive.org/details/tucows_302448_Penokio_OutLynk) | `Outlynk4.exe` | Does not start | This is a desktop Windows program, not a Pocket PC one. |
| [PenReader](https://archive.org/details/tucows_32570_PenReader) | `prhpc_eng_demo.exe` | Does not start | This is a desktop Windows program, not a Pocket PC one. |
| [PerfectPitch](https://archive.org/details/tucows_286595_PerfectPitch) | `PerfectPitchDemoPPC.zip` | Does not start | The game stopped on something the emulator cannot do yet. |
| [Periodic Table](https://archive.org/details/tucows_48686_Periodic_Table) | `PTableHPC.exe` | Does not start | This is a desktop Windows program, not a Pocket PC one. |
| [PhotoView](https://archive.org/details/tucows_360352_PhotoView) | `photoview.zip` | Does not start | The game needs a file that comes with the Pocket PC itself. Missing: `mfcce300.dll!#644` |
| [Pic and Ring](https://archive.org/details/tucows_320312_Pic_and_Ring) | `picring.zip` | Does not start | The game needs a file that comes with the Pocket PC itself. Missing: `mfcce300.dll!#644` |
| [PicoWebServer](https://archive.org/details/tucows_200784_PicoWebServer) | `PicoWebServer.exe` | Does not start | This is a desktop Windows program, not a Pocket PC one. |
| [PixScreen Lite](https://archive.org/details/tucows_158103_PixScreen_Lite) | `pscelite.exe` | Does not start | This is a desktop Windows program, not a Pocket PC one. |
| [pkHpc](https://archive.org/details/tucows_32213_pkHpc) | `pkHpc.zip` | Does not start |  |
| [Plus Today Plugin MFC](https://archive.org/details/plus-today-plugin-mfc) | `PocketPlus/ActionSender.exe` | Does not start |  |
| [Plus Today Plugin MFC](https://archive.org/details/plus-today-plugin-mfc) | `PocketPlus/AltTab.exe` | Does not start | The game stopped on something the emulator cannot do yet. Missing: `COREDLL.dll!CharLowerBuffW` |
| [Pocket Assistant](https://archive.org/details/tucows_217346_Pocket_Assistant) | `PocketAssistantEnglish120.ZIP` | Does not start | The game needs a file that comes with the Pocket PC itself. Missing: `mfcce300.dll!#644` |
| [Pocket Controller](https://archive.org/details/tucows_210581_Pocket_Controller) | `PKTCtrlDmoSetup.exe` | Does not start | This is a desktop Windows program, not a Pocket PC one. |
| [Pocket DB Pro Edition](https://archive.org/details/tucows_151111_Pocket_DB_Pro_Edition) | `pkdb32sa.exe` | Does not start | This is a desktop Windows program, not a Pocket PC one. |
| [Pocket DB Pro Edition](https://archive.org/details/tucows_151111_Pocket_DB_Pro_Edition) | `pkdb32s3.exe` | Does not start | This is a desktop Windows program, not a Pocket PC one. |
| [Pocket Diary](https://archive.org/details/tucows_287818_Pocket_Diary) | `PocDiary_Share.exe` | Does not start | The game's program file cannot be loaded. |
| [Pocket Dictate](https://archive.org/details/tucows_319047_Pocket_Dictate) | `pdictate.zip` | Does not start | The game needs a file that is not in the package. |
| [Pocket Dictate](https://archive.org/details/tucows_341420_Pocket_Dictate) | `pdictate.zip` | Does not start | The game needs a file that is not in the package. |
| [Pocket File Finder](https://archive.org/details/tucows_200755_Pocket_File_Finder) | `PFFARM.exe` | Does not start |  |
| [Pocket Game Trainer](https://archive.org/details/tucows_303783_Pocket_Game_Trainer) | `PGTDemo.zip` | Does not start | The game needs a file that comes with the Pocket PC itself. Missing: `mfcce300.dll!#644` |
| [Pocket Goo](https://archive.org/details/tucows_226729_Pocket_Goo) | `PocketGooDemo.exe` | Does not start | This is a desktop Windows program, not a Pocket PC one. |
| [Pocket Image](https://archive.org/details/tucows_216359_Pocket_Image) | `pkim20sa.zip` | Does not start | This is a desktop Windows program, not a Pocket PC one. |
| [Pocket Jongg](https://archive.org/details/tucows_32164_Pocket_Jongg) | `pjv30demo.exe` | Does not start | The game's program file cannot be loaded. |
| [Pocket Lemmings](https://archive.org/details/pocket-lemmings) | `PocketLemmings.zip` | Does not start | Missing: `COREDLL.dll!GetOpenFileNameW` |
| [Pocket Painter](https://archive.org/details/tucows_209445_Pocket_Painter) | `PocketPainterEnglish150.ZIP` | Does not start | The game needs a file that comes with the Pocket PC itself. Missing: `mfcce300.dll!#644` |
| [Pocket PC 2003 Upgrade for Compaq iPAQ 3970](https://archive.org/details/3970upgrade_202108) | `psshutxp_eng.xscale.cab` | Does not start | The game needs a file that is not in the package. Missing: `COREDLL.dll!SHCreateShortcut`, `IMGDECMP.DLL!#1` |
| [Pocket Piano](https://archive.org/details/tucows_212520_Pocket_Piano) | `PocketPianoEnglish100.ZIP` | Does not start | The game needs a file that comes with the Pocket PC itself. Missing: `mfcce300.dll!#644` |
| [Pocket Plan](https://archive.org/details/tucows_281294_Pocket_Plan) | `pocketplan.zip` | Does not start | This is a desktop Windows program, not a Pocket PC one. |
| [Pocket Property Manager](https://archive.org/details/tucows_239407_Pocket_Property_Manager) | `propertysetup.zip` | Does not start | This is a desktop Windows program, not a Pocket PC one. |
| [Pocket RTA](https://archive.org/details/tucows_318545_Pocket_RTA) | `PocketRTADemo.exe` | Does not start | The game's program file cannot be loaded. |
| [Pocket Sales Manager](https://archive.org/details/tucows_251398_Pocket_Sales_Manager) | `PocketSalesManagerDemo.exe` | Does not start |  |
| [Pocket Streets 2001 for Windows CE](https://archive.org/details/pocket-streets-2001-for-windows-ce) | `P.RAP_ARM.CAB` | Does not start | Missing: `AYGSHELL.dll!#69`, `DocList.dll!DocList_Create` |
| [Pocket Text Finder](https://archive.org/details/tucows_315290_Pocket_Text_Finder) | `PocketTextFinderSetup.exe` | Does not start | The game needs a file that comes with the Pocket PC itself. Missing: `mfcce300.dll!#644` |
| [Pocket TurboDoc Electronic Medical Record](https://archive.org/details/tucows_365541_Pocket_TurboDoc_Electronic_Medical_Record) | `All_TurboDocPPC_2003Free.exe` | Does not start | This is a desktop Windows program, not a Pocket PC one. |
| [Pocket TV Pro ACER EN](https://archive.org/details/pocket-tv-pro-acer-en) | `PocketTV Pro ACER EN.zip` | Does not start | The game needs a file that is not in the package. Missing: `coredll.dll!CeGetThreadQuantum` |
| [Pocket Wallet](https://archive.org/details/tucows_54499_Pocket_Wallet) | `PWalletPPCSetup.exe` | Does not start | Missing: `COREDLL.dll!RegisterClipboardFormatW`, `COREDLL.dll!ImageList_ReplaceIcon`, `COREDLL.dll!SHGetShortcutTarget` |
| [PocketBar 1.2](https://archive.org/details/pocketbar12) | `pocketbar1.2.zip` | Does not start | This is a .NET Compact Framework program, which PocketPC3630 cannot run. |
| [PocketChess](https://archive.org/details/tucows_32162_PocketChess) | `PocketChessForPPC.exe` | Does not start |  |
| [PocketCommander](https://archive.org/details/tucows_251950_PocketCommander) | `pocketcommander0441b.zip` | Does not start | The game needs a file that comes with the Pocket PC itself. Missing: `mfcce300.dll!#644` |
| [PocketDOS](https://archive.org/details/tucows_49074_PocketDOS) | `PocketDOS_107.exe` | Does not start | The game needs a file that is not in the package. |
| [PocketDOS 1.12.2. ARM PPC](https://archive.org/details/pocket-dos-1.12.2) | `PocketDOS.v1.12.2.ARM.PPC.Cracked-Sarge.zip` | Does not start | The game stopped on something the emulator cannot do yet. Missing: `COREDLL.dll!CharUpperBuffW` |
| [PocketGCC](https://archive.org/details/src_all-20040803-gcc-3.3.3-binutils-2.13.2.1-newlib-1.11.0-pthreads-20021104) | `pocketgcc-1.50-20031108-arm.zip` | Does not start | Missing: `COREDLL.dll!GetStdioPathW`, `COREDLL.dll!towupper`, `COREDLL.dll!fputwc` |
| [PocketGCC](https://archive.org/details/src_all-20040803-gcc-3.3.3-binutils-2.13.2.1-newlib-1.11.0-pthreads-20021104) | `src_all-20040803-gcc-3.3.3-binutils-2.13.2.1-newlib-1.11.0-pthreads-20021104.rar` | Does not start | Missing: `libc.dll!_startup` |
| [PocketJam](https://archive.org/details/tucows_299367_PocketJam) | `PocketJam_Install.exe` | Does not start | The game needs a file that comes with the Pocket PC itself. Missing: `mfcce300.dll!#644` |
| [PocketMap Annual Premiere Nationwide](https://archive.org/details/tucows_223424_PocketMap_Annual_Premiere_Nationwide) | `PocketMapTrial102.exe` | Does not start | The game needs a file that comes with the Pocket PC itself. Missing: `mfcce300.dll!#644` |
| [PocketMarkup](https://archive.org/details/tucows_252429_PocketMarkup) | `pmu-arm-1.1.0.exe` | Does not start |  |
| [PocketMySQLAdmin](https://archive.org/details/tucows_269775_PocketMySQLAdmin) | `PPC_PMASetup.exe` | Does not start | The game needs a file that comes with the Pocket PC itself. Missing: `mfcce300.dll!#644` |
| [PocketRx](https://archive.org/details/tucows_328269_PocketRx) | `prx2003en.zip` | Does not start | This is a desktop Windows program, not a Pocket PC one. |
| [PocketSol](https://archive.org/details/tucows_279494_PocketSol) | `psol20.zip` | Does not start | The game needs a file that is not in the package. |
| [PokerHand](https://archive.org/details/tucows_220194_PokerHand) | `PokerHandInst176c.zip` | Does not start | The game stopped on something the emulator cannot do yet. |
| [POWDER Roguelike Complete Archive](https://archive.org/details/powder-magicsam-pandora) | `WinCE/powder-setup-winCE winmo pocketpc.zip` | Does not start | The game stopped on something the emulator cannot do yet. Missing: `SDL.dll!SDL_Quit`, `SDL.dll!SDL_Init`, `SDL.dll!SDL_SetVideoMode` |
| [POWDER Roguelike Complete Archive](https://archive.org/details/powder-magicsam-pandora) | `WinCE/powder-barebone-winCE winmo pocketpc.zip` | Does not start | The game stopped on something the emulator cannot do yet. Missing: `SDL.dll!SDL_Quit`, `SDL.dll!SDL_Init`, `SDL.dll!SDL_SetVideoMode` |
| [Power Supply Properties](https://archive.org/details/tucows_32605_Power_Supply_Properties) | `PowerSupplyHPC.zip` | Does not start |  |
| [PowerOff](https://archive.org/details/tucows_218943_PowerOff) | `PowerOff10.zip` | Does not start |  |
| [PowerVR Demos](https://archive.org/details/pvr-racer) | `OGLESLighting.zip` | Does not start | Missing: `COREDLL.dll!MonitorFromWindow`, `COREDLL.dll!GetMonitorInfo` |
| [PowerVR Racer (Ridge Racer OpenGL MBX ES 1.1 tech demo) for Dell Axim x50v x51v](https://archive.org/details/powervr-racer) | `PVR_Racer.zip` | Does not start | The game stopped on something the emulator cannot do yet. Missing: `COREDLL.dll!MonitorFromWindow`, `COREDLL.dll!GetMonitorInfo` |
| [PrinterOn PocketWhere](https://archive.org/details/tucows_274453_PrinterOn_PocketWhere) | `pwce_install.exe` | Does not start | This is a desktop Windows program, not a Pocket PC one. |
| [Psion Workabout Pro G1 firmware](https://archive.org/details/psion-workabout-pro-g1-firmware) | `Windows Mobile 2003-H146p.zip` | Does not start |  |
| [QuickContacts](https://archive.org/details/tucows_214829_QuickContacts) | `QuickContacts.zip` | Does not start | The game needs a file that comes with the Pocket PC itself. Missing: `mfcce300.dll!#644` |
| [QuickTask](https://archive.org/details/tucows_47022_QuickTask) | `QuickTask.ppcpro_sh3.CAB` | Does not start | The game needs a file that comes with the Pocket PC itself. Missing: `mfcce211.dll!#666` |
| [Quiz Master](https://archive.org/details/tucows_210154_Quiz_Master) | `quizmasterinstaller.zip` | Does not start | This is a .NET Compact Framework program, which PocketPC3630 cannot run. |
| [Rainbow](https://archive.org/details/tucows_32217_Rainbow) | `Rainbow20Demo.exe` | Does not start | The game's program file cannot be loaded. |
| [Random Number Generator for Pocket PCs](https://archive.org/details/randomnumgenwm) | `randomnumgenwm_v1.2.cab` | Does not start | This is a .NET Compact Framework program, which PocketPC3630 cannot run. |
| [Rayman 2002 arm CD ver full En & Jp](https://archive.org/details/rayman-1.arm.-cdver.full.-en) | `Rayman1.arm.CDver.full.En.zip` | Does not start | The game stopped on something the emulator cannot do yet. |
| [Rayman 2002 arm CD ver full En & Jp](https://archive.org/details/rayman-1.arm.-cdver.full.-en) | `Rayman1.arm.CDver.full.Japan.zip` | Does not start | The game stopped on something the emulator cannot do yet. |
| [Rayman Ultimate WM2003 & WM5](https://archive.org/details/rayman-ultimate-wm-5) | `RaymanUltimateARM wm2003.zip` | Timed out | Did not finish starting within 120 seconds Missing: `zlib.dll!gzopen`, `zlib.dll!gzread`, `zlib.dll!gzclose` |
| [Rayman Ultimate WM2003 & WM5](https://archive.org/details/rayman-ultimate-wm-5) | `Rayman_Ultimate_WM5.rar` | Timed out | Did not finish starting within 120 seconds |
| [Reckless Thief ARM](https://archive.org/details/reckless-thief-arm) | `RecklessThief ARM.zip` | Does not start | The game stopped on something the emulator cannot do yet. |
| [Recordian](https://archive.org/details/tucows_32438_Recordian) | `RecordianPPC30Day.exe` | Does not start | The game stopped on something the emulator cannot do yet. Missing: `COREDLL.dll!ImageList_ReplaceIcon`, `COREDLL.dll!SHGetShortcutTarget`, `COREDLL.dll!GetTextFaceW` |
| [Resco Explorer 2010 8.10](https://archive.org/details/RescoExplorer_201810) | `Resco Explorer.cab` | Does not start | The game needs a file that is not in the package. Missing: `AYGSHELL.dll!#69` |
| [Rosesams Screen Shooter](https://archive.org/details/rsss_20200914) | `RSSS.zip` | Does not start | This is a .NET Compact Framework program, which PocketPC3630 cannot run. |
| [RUU Excalibur HTC WWE 1.11.405.4 4.1.13.28 02.63.90](https://archive.org/details/ruu-excalibur-htc-wwe-1.11.405.4-4.1.13.28-02.63.90) | `RUU_Excalibur_HTC_WWE_1.11.405.4_4.1.13.28_02.63.90_Ship.exe` | Does not start | This is a desktop Windows program, not a Pocket PC one. |
| [Samsung Power Calc](https://archive.org/details/samsung-power-calc) | `Samsung PowerCalc.cab` | Does not start | The game needs a file that is not in the package. Missing: `toolhelp.dll!CreateToolhelp32Snapshot`, `toolhelp.dll!Process32First`, `toolhelp.dll!CloseToolhelp32Snapshot` |
| [Secure File PDA Backup MultiMediaCard](https://archive.org/details/PDABACKUP) | `PDABACKUP.ZIP` | Does not start | The game needs a file that comes with the Pocket PC itself. Missing: `mfcce300.dll!#644` |
| [Shopaholic](https://archive.org/details/tucows_325949_Shopaholic) | `shopaholic.exe` | Does not start |  |
| [Shortcad 2009-07-12-1.2.60 for Windows Mobile](https://archive.org/details/shortcad-2009-07-12-1.2.60) | `shortcad-2009-07-12-1.2.60.cab` | Does not start | The game stopped on something the emulator cannot do yet. |
| [ShortCAD.Lite 1.2 WM5](https://archive.org/details/shortcadlite-1.2-wm5) | `ShortCAD.Lite.1.2_WM5.cab` | Does not start | The game stopped on something the emulator cannot do yet. |
| [SMath Studio Handheld 0.90 for Pocket PC](https://archive.org/details/smath-studio-handheld-0-90-setup) | `SMathStudioHandheld.0_90.Setup.cab` | Does not start | This is a .NET Compact Framework program, which PocketPC3630 cannot run. |
| [SnapIt](https://archive.org/details/tucows_32521_SnapIt) | `SnapIt_Install.exe` | Does not start | This is a desktop Windows program, not a Pocket PC one. |
| [snowedin7_v1_00_wsp_qvga](https://archive.org/details/snowedin7_v1_00_wsp_qvga) | `snowedin7_v1_00_wsp_qvga.zip` | Does not start | The game needs a file that is not in the package. |
| [SoftReset](https://archive.org/details/tucows_221462_SoftReset) | `SoftReset10.zip` | Does not start |  |
| [Soo Calculator (Lazen)](https://archive.org/details/tucows_213724_Soo_Calculator_Lazen) | `SooCalcDemo_ARM.zip` | Does not start | The game needs a file that comes with the Pocket PC itself. Missing: `mfcce300.dll!#644` |
| [Sp 30003( BT PPC 1.6.0.0700 AV Installer 02 16 05)](https://archive.org/details/sp30003btppc1.6.0.0700avinstaller021605) | `Broadcom Corporation BT-PPC AV.cab` | Does not start |  |
| [Sp 30003( BT PPC 1.6.0.0700 AV Installer 02 16 05)](https://archive.org/details/sp30003btppc1.6.0.0700avinstaller021605) | `sp30003(BT-PPC 1.6.0.0700 AV Installer-02-16-05).exe` | Does not start | This is a desktop Windows program, not a Pocket PC one. |
| [sp28733](https://archive.org/details/sp28733) | `Wifi Upgrade.exe` | Does not start | This is a desktop Windows program, not a Pocket PC one. |
| [sp28733](https://archive.org/details/sp28733) | `Rom upgrade.exe` | Does not start | This is a desktop Windows program, not a Pocket PC one. |
| [SpaceMaker](https://archive.org/details/tucows_276409_SpaceMaker) | `SpaceMaker_Trial.exe` | Does not start | This is a desktop Windows program, not a Pocket PC one. |
| [Spanish-English and English-Spanish dictionary](https://archive.org/details/tucows_253536_Spanish-English_and_English-Spanish_dictionary) | `SpanEngl_SmL_CE.zip` | Does not start | This is a desktop Windows program, not a Pocket PC one. |
| [Spb Full Screen Keyboard](https://archive.org/details/tucows_281220_Spb_Full_Screen_Keyboard) | `fullscreenkeyboard2_setup.exe` | Does not start | The game needs a file that comes with the Pocket PC itself. Missing: `mfcce300.dll!#644` |
| [Spb Lines](https://archive.org/details/tucows_254251_Spb_Lines) | `lines_setup.exe` | Does not start | The game needs a file that comes with the Pocket PC itself. Missing: `mfcce300.dll!#644` |
| [Spb Pocket Plus](https://archive.org/details/tucows_302503_Spb_Pocket_Plus) | `pocketplus_setup.exe` | Does not start | The game needs a file that comes with the Pocket PC itself. Missing: `mfcce300.dll!#644` |
| [SpreadCE 3.14 2023-06-10](https://archive.org/details/spreadce-3.14-2023-06-10) | `spreppcs.zip` | Does not start | The game needs a file that is not in the package. Missing: `COREDLL.dll!towupper`, `COREDLL.dll!GetVersionEx` |
| [Sprite Backup Premium v3.1.4 cracked](https://archive.org/details/sprite_backup_premium_3.1.4) | `sprite_backup_premium_v3.1.4_reg.rar` | Does not start | The game needs a file that comes with the Pocket PC itself. Missing: `mfcce300.dll!#644` |
| [sshCE](https://archive.org/details/tucows_56835_sshCE) | `sshCE1pro.exe` | Timed out | Did not finish starting within 120 seconds |
| [Star Tap ARM](https://archive.org/details/star-tap-arm) | `StarTap ARM.zip` | Does not start |  |
| [StyleTap platform v.1.1.020 -PocketPC/WinMobile- cracked (CBD)](https://archive.org/details/style-tap.v.-1.1.020.-cbd) | `StyleTap.v.1.1.020.SP.CBD.ZIP` | Does not start |  |
| [SyncExpress 2002](https://archive.org/details/tucows_224685_SyncExpress_2002) | `syex2k2setup.exe` | Does not start | This is a desktop Windows program, not a Pocket PC one. |
| [SyncTalk FX](https://archive.org/details/tucows_219053_SyncTalk_FX) | `SyncTalkWindows2000.exe` | Does not start | This is a desktop Windows program, not a Pocket PC one. |
| [SyncTalk FX (WindowsCE 2.11 HPC Full Screen)](https://archive.org/details/tucows_219056_SyncTalk_FX_WindowsCE_2.11_HPC_Full_Screen) | `SyncTalk_HPC211FULL.exe` | Does not start | This is a desktop Windows program, not a Pocket PC one. |
| [Syntrax](https://archive.org/details/tucows_317198_Syntrax) | `syntrax_arm.exe` | Does not start | The game needs a file that comes with the Pocket PC itself. Missing: `mfcce300.dll!#644` |
| [T-Plus Bridge](https://archive.org/details/tucows_183965_T-Plus_Bridge) | `TPLUS_POCKET_setup.exe` | Does not start |  |
| [T-Plus Bridge](https://archive.org/details/tucows_68425_T-Plus_Bridge) | `TPLUS_HPC_setup.exe` | Does not start |  |
| [Targus Stowaway Keyboard Driver](https://archive.org/details/targus-stowaway-keyboard-driver) | `Targus Stowaway Keyboard Driver.exe` | Does not start | This is a desktop Windows program, not a Pocket PC one. |
| [TCPMP (v0.71) (Windows Mobile Port)](https://archive.org/details/tcpmp.arm_sp) | `tcpmp.arm_sp.cab` | Does not start |  |
| [TCPMP Core Desktop Installer Extracted](https://archive.org/details/tcpmp-cabs) | `TCPMP/tcpmp.arm_sp.cab` | Does not start |  |
| [TCSync](https://archive.org/details/tucows_263713_TCSync) | `TCSync.exe` | Does not start | This is a desktop Windows program, not a Pocket PC one. |
| [TimeBandit](https://archive.org/details/tucows_231620_TimeBandit) | `TimeBandit.exe` | Does not start | The game needs a file that comes with the Pocket PC itself. Missing: `mfcce300.dll!#644` |
| [TimeReporter](https://archive.org/details/tucows_32448_TimeReporter) | `trce.exe` | Does not start | The game's program file cannot be loaded. |
| [TimeReporter for Timeslips](https://archive.org/details/tucows_32449_TimeReporter_for_Timeslips) | `tr4ts.exe` | Does not start | The game's program file cannot be loaded. |
| [Tiny TeamTalk](https://archive.org/details/tucows_353860_Tiny_TeamTalk) | `TinyTeamTalk_v1.1.exe` | Does not start | The game needs a file that comes with the Pocket PC itself. Missing: `mfcce300.dll!#644` |
| [Tome Raider 3 ARM](https://archive.org/details/tome-raider-3-arm) | `TomeRaider3 ARM.zip` | Does not start | The game needs a file that comes with the Pocket PC itself. Missing: `coredll.dll!AFS_CreateDirectoryW` |
| [TOSHIBA GENIO-SPEECH Version 1.11 cab](https://archive.org/details/genio-speech) | `GENIO-SPEECH.cab` | Does not start | The game needs a file that is not in the package. Missing: `COREDLL.dll!ImageList_ReplaceIcon`, `COREDLL.dll!DestroyIcon`, `SpeechOut.dll!??0SpeakOut@@QAA@XZ` |
| [TrueTerm CE Culinary Dictionary English/Dutch](https://archive.org/details/tucows_180913_TrueTerm_CE_Culinary_Dictionary_English_Dutch) | `TTWinCEEnglishDutchEnglish.zip` | Does not start | The game stopped on something the emulator cannot do yet. Missing: `mfcce211.dll!#612` |
| [TrueTerm CE Culinary Dictionary English/French](https://archive.org/details/tucows_180918_TrueTerm_CE_Culinary_Dictionary_English_French) | `TTWinCEEnglishFrench.zip` | Does not start | The game stopped on something the emulator cannot do yet. Missing: `mfcce211.dll!#612` |
| [TrueTerm CE Culinary Dictionary English/Italian](https://archive.org/details/tucows_180924_TrueTerm_CE_Culinary_Dictionary_English_Italian) | `TTWinCEEnglishItalianEnglish.zip` | Does not start | The game stopped on something the emulator cannot do yet. Missing: `mfcce211.dll!#612` |
| [TrueTerm CE Culinary Dictionary English/Portuguese](https://archive.org/details/tucows_180928_TrueTerm_CE_Culinary_Dictionary_English_Portuguese) | `TTWinCEEnglishPortugueseEnglish.zip` | Does not start | The game stopped on something the emulator cannot do yet. Missing: `mfcce211.dll!#612` |
| [TrueTerm CE Culinary Dictionary English/Spanish](https://archive.org/details/tucows_180980_TrueTerm_CE_Culinary_Dictionary_English_Spanish) | `TTWinCEEnglishSpanishEnglish.zip` | Does not start | The game stopped on something the emulator cannot do yet. Missing: `mfcce211.dll!#612` |
| [Tweak](https://archive.org/details/tucows_45524_Tweak) | `TweakCE.exe` | Does not start | The game needs a file that comes with the Pocket PC itself. Missing: `mfcce300.dll!#644` |
| [Vectorsoft Draw 3.01 Pocket PC (Extracted)](https://archive.org/details/vsdraw301-extracted) | `vsdraw.sa.cab` | Does not start | The game needs a file that comes with the Pocket PC itself. Missing: `mfcce300.dll!#644` |
| [Vectorsoft Draw 3.01 Pocket PC (Extracted)](https://archive.org/details/vsdraw301-extracted) | `vsdraw.arm.cab` | Does not start | The game needs a file that comes with the Pocket PC itself. Missing: `mfcce300.dll!#644` |
| [Veo Photo Traveler For Pocket PC Driver Installation](https://archive.org/details/veo-photo-traveler-for-pocket-pc-driver) | `Veo Photo Traveler for Pocket PC Setup/veotraveler.arm.CAB` | Does not start | The game needs a file that comes with the Pocket PC itself. Missing: `mfcce300.dll!#644` |
| [Vexed. 1.9.8.423. PPC. WM 5.arm ( 2)](https://archive.org/details/vexed-ppc) | `Vexed PPC.cab` | Does not start | The game needs a file that is not in the package. Missing: `COREDLL.dll!CharNextW` |
| [vlc-0.8.5-svn-20060214-0130-wince-arm](https://archive.org/details/vlc-0.8.5-svn-20060214-0130-wince-arm) | `vlc-0.8.5-svn-20060214-0130-wince.zip` | Does not start |  |
| [VTI Pocket Emulator v2.4 for ARM only (Extracted)](https://archive.org/details/vti-pocket-emulator-v2.4) | `TI.CAB` | Does not start | The game needs a file that comes with the Pocket PC itself. Missing: `mfcce300.dll!#644` |
| [vxHpc](https://archive.org/details/tucows_32229_vxHpc) | `vxHpc.zip` | Does not start |  |
| [WCE EuroConverter](https://archive.org/details/tucows_240806_WCE_EuroConverter) | `ppc3_mips.zip` | Does not start | The game stopped on something the emulator cannot do yet. Missing: `mfcce211.dll!#631` |
| [WCE EuroConverter](https://archive.org/details/tucows_240806_WCE_EuroConverter) | `ppc3_sh3.zip` | Does not start | The game stopped on something the emulator cannot do yet. Missing: `mfcce211.dll!#666` |
| [WebToGo SMS/Phone](https://archive.org/details/tucows_353082_WebToGo_SMS_Phone) | `WTGPhone.zip` | Does not start | The game needs a file that comes with the Pocket PC itself. Missing: `mfcce300.dll!#644` |
| [Werd Jot](https://archive.org/details/tucows_32571_Werd_Jot) | `WerdJot.zip` | Does not start | The game needs a file that comes with the Pocket PC itself. Missing: `mfcce20.dll!#516` |
| [WinCE 2.0 Service Pack](https://archive.org/details/tucows_32561_WinCE_2.0_Service_Pack) | `Sp1.exe` | Does not start | The game's program file cannot be loaded. |
| [WinCE 3.0 Service Pack 1](https://archive.org/details/tucows_32560_WinCE_3.0_Service_Pack_1) | `HPC30SP1.zip` | Does not start | This is a desktop Windows program, not a Pocket PC one. |
| [wincecabs3](https://archive.org/details/wincecabs3) | `WinCEPaint/ce_paint.StrongARM.CAB` | Does not start | The game needs a file that comes with the Pocket PC itself. Missing: `mfcce300.dll!#644` |
| [WinceZip](https://archive.org/details/tucows_267215_WinceZip) | `WinceZipEnglish100.zip` | Does not start | The game needs a file that comes with the Pocket PC itself. Missing: `mfcce300.dll!#644` |
| [Windows 10 SDK (Version 10.0.16299.91) and Microsoft Emulator for Windows 10 Mobile (Version 10.0.15254.1)](https://archive.org/details/windows10sdkmsewindows10mobile) | `EmulatorSetup.exe` | Does not start | This is a desktop Windows program, not a Pocket PC one. |
| [Windows 10 SDK (Version 10.0.16299.91) and Microsoft Emulator for Windows 10 Mobile (Version 10.0.15254.1)](https://archive.org/details/windows10sdkmsewindows10mobile) | `winsdksetup.exe` | Does not start | This is a desktop Windows program, not a Pocket PC one. |
| [Windows CE Japanese Pocket PC Programs](https://archive.org/details/windows-ce-japanese-pocket-pc-programs) | `Windows CE Japanese Pocket PC Programs.zip` | Does not start | Missing: `COREDLL.dll!RegisterClipboardFormatW`, `COREDLL.dll!SHGetShortcutTarget`, `AYGSHELL.dll!#69` |
| [Windows CE Services v2.x to v2.1](https://archive.org/details/tucows_32563_Windows_CE_Services_v2.x_to_v2.1) | `wces21.exe` | Does not start | The game's program file cannot be loaded. |
| [Windows CE Services v2.x to v2.2](https://archive.org/details/tucows_32564_Windows_CE_Services_v2.x_to_v2.2) | `WindowsCEServices22.zip` | Does not start | The game's program file cannot be loaded. |
| [Windows Device Recovery Tool](https://archive.org/details/wdrtinstaller_202512) | `FFULoaderDriver.exe` | Does not start | This is a desktop Windows program, not a Pocket PC one. |
| [Windows Device Recovery Tool](https://archive.org/details/wdrtinstaller_202512) | `WindowsDeviceRecoveryToolInstaller.exe` | Does not start | This is a desktop Windows program, not a Pocket PC one. |
| [Windows Media Player 1.2 for Handheld PC](https://archive.org/details/WMNAll-HPC) | `naut640x240.ARM.CAB` | Does not start | The game stopped on something the emulator cannot do yet. |
| [Windows Media Player 1.2 for Handheld PC](https://archive.org/details/WMNAll-HPC) | `naut640x480.ARM.CAB` | Does not start | The game stopped on something the emulator cannot do yet. |
| [Windows Media Player 7.1 for Pocket PC 3.0](https://archive.org/details/windows-media-player-7.1-for-pocket-pc-3.0) | `wolf.ARM.CAB` | Does not start | Missing: `COREDLL.dll!GetTempPathW` |
| [Windows Media Player for Palm-size PC (Extracted)](https://archive.org/details/windows-media-player-for-palm-size-pc) | `wyvrncpq.mips.CAB` | Does not start | The game stopped on something the emulator cannot do yet. |
| [Windows Media Player for Palm-size PC (Extracted)](https://archive.org/details/windows-media-player-for-palm-size-pc) | `wyvrncso.mips.CAB` | Does not start | The game stopped on something the emulator cannot do yet. |
| [Windows Media Player for Pocket PC - Technology Preview Edition 2](https://archive.org/details/windowsmediaplayerforpocketpc-technologypreviewedition12extracted) | `Windows Media Player for Pocket PC - Technology Preview Edition/WMPPPC.exe` | Does not start | This is a desktop Windows program, not a Pocket PC one. |
| [Windows Media Player for Pocket PC 8.0 (Extracted)](https://archive.org/details/windows-media-player-for-pocket-pc-8.0) | `wolfRAM.ARM.CAB` | Does not start | Missing: `COREDLL.dll!GetTempPathW` |
| [Windows Media Player for Pocket PC 8.0 (Extracted)](https://archive.org/details/windows-media-player-for-pocket-pc-8.0) | `wolfROM.ARM.CAB` | Does not start | Missing: `COREDLL.dll!GetTempPathW` |
| [Windows Mobile application archive](https://archive.org/details/windowsmobile) | `CapSure_PPC_v1.11g.cab` | Does not start | This is a .NET Compact Framework program, which PocketPC3630 cannot run. |
| [Windows mobile apps](https://archive.org/details/windows_mobile_apps) | `windows mobile apps/AntiqueTetrisCE.ARM.PocketPC (Version 3.0) (English).CAB` | Does not start | The game needs a file that comes with the Pocket PC itself. Missing: `mfcce300.dll!#644` |
| [Windows mobile apps](https://archive.org/details/windows_mobile_apps) | `windows mobile apps/SploitDemo.Arm.Cab` | Does not start | The game needs a file that comes with the Pocket PC itself. Missing: `mfcce300.dll!#644` |
| [Windows Mobile Device Center](https://archive.org/details/windows-mobile-device-center) | `drvupdate-amd64.exe` | Does not start | This is a desktop Windows program, not a Pocket PC one. |
| [Windows Mobile Device Center (WMDC) Fix for Windows 10/11](https://archive.org/details/wmdc-win-10-11) | `WMDC_Win10-11.zip` | Does not start | This is a desktop Windows program, not a Pocket PC one. |
| [Windows Mobile Device Center Windows 10](https://archive.org/details/wmdc-pack) | `WMDC-Pack.zip` | Does not start | This is a desktop Windows program, not a Pocket PC one. |
| [Windows Mobile SDKs](https://archive.org/details/windows-mobile-sdks-2023) | `vs_emulator_x64_vista.exe` | Does not start | This is a desktop Windows program, not a Pocket PC one. |
| [Windows Mobile SDKs](https://archive.org/details/windows-mobile-sdks-2023) | `vs_emulator.exe` | Does not start | This is a desktop Windows program, not a Pocket PC one. |
| [Windows mobile software](https://archive.org/details/windowsmobileapps) | `AntiqueTetrisCE.ARM.PocketPC (Version 3.0) (English).CAB` | Does not start | The game needs a file that comes with the Pocket PC itself. Missing: `mfcce300.dll!#644` |
| [Windows mobile software](https://archive.org/details/windowsmobileapps) | `SploitDemo.Arm.Cab` | Does not start | The game needs a file that comes with the Pocket PC itself. Missing: `mfcce300.dll!#644` |
| [Windows mobile UITweaker](https://archive.org/details/UITweaker) | `UITweaker.cab` | Does not start | The game needs a file that comes with the Pocket PC itself. Missing: `coredll.dll!AFS_CreateDirectoryW` |
| [Windows Phone 7/8 Games Collection: PDALife Archive (A,B,C,D)](https://archive.org/details/windows-phone-pdalife) | `Desperated Housewives.cab` | Does not start | The game needs a file that is not in the package. |
| [Windows Phone Desktop App](https://archive.org/details/windows-phone-desktop-app) | `WindowsPhone1.exe` | Does not start | This is a desktop Windows program, and no Pocket PC game could be taken out of it. |
| [Windows Phone SDK 7.8 and 8.0 (Russian)](https://archive.org/details/WindowsPhoneSDK-7.8) | `Windows Phone SDK 7.8 Update/Packages/Images_7.8/rus/cab3.cab` | Does not start | This is a desktop Windows program, not a Pocket PC one. |
| [Windows Pocket PC 2000 games & programms](https://archive.org/details/windows-pocket-pc-2000) | `WindowsPocketPC2000.rar` | Does not start |  |
| [Windows Pocket PC 2002 Stuff](https://archive.org/details/WindowsPocketPCStuff-2002) | `ARM_CABs/Games/Dots-1.1.1.cab` | Does not start | The game stopped on something the emulator cannot do yet. |
| [WindowsMediaPlayer Playlist Converter](https://archive.org/details/tucows_320130_WindowsMediaPlayer_Playlist_Converter) | `vbPlaylistConverter.zip` | Does not start | This is a desktop Windows program, not a Pocket PC one. |
| [WinWAP](https://archive.org/details/tucows_313803_WinWAP) | `WinWAP3P.exe` | Does not start | This is a desktop Windows program, not a Pocket PC one. |
| [Wireless Sales](https://archive.org/details/tucows_275201_Wireless_Sales) | `WirelessSales1.zip` | Does not start | This is a desktop Windows program, not a Pocket PC one. |
| [Wireless Sales Wireless Sales](https://archive.org/details/tucows_274813_Wireless_Sales_Wireless_Sales) | `WirelessSales.zip` | Does not start | This is a desktop Windows program, not a Pocket PC one. |
| [WMCave - SFCave for Windows Mobile](https://archive.org/details/wmcave) | `wmcave.cab` | Does not start | This is a .NET Compact Framework program, which PocketPC3630 cannot run. |
| [WMCave - SFCave for Windows Mobile](https://archive.org/details/wmcave) | `wmcave_gs.cab` | Does not start | This is a .NET Compact Framework program, which PocketPC3630 cannot run. |
| [Wndows Mobile 6.5 Stuff](https://archive.org/details/WindowsMobileStuff-6) | `destr0_VOIPandSIP_Config-1.1_WM61.CAB` | Does not start | This is a .NET Compact Framework program, which PocketPC3630 cannot run. |
| [Wndows Mobile 6.5 Stuff](https://archive.org/details/WindowsMobileStuff-6) | `SkypeMobileSP.CAB` | Does not start | Missing: `COREDLL.dll!SendMessageTimeout`, `COREDLL.dll!UnregisterClassW` |
| [XAudio MP3 Player](https://archive.org/details/tucows_32632_XAudio_MP3_Player) | `xaudio-wince.exe` | Does not start | The game stopped on something the emulator cannot do yet. |
| [Xircuit Theme Generator](https://archive.org/details/tucows_232439_Xircuit_Theme_Generator) | `xtg.zip` | Does not start | This is a desktop Windows program, not a Pocket PC one. |
| [Zerama Remote](https://archive.org/details/tucows_328193_Zerama_Remote) | `zerama.zip` | Does not start | This is a .NET Compact Framework program, which PocketPC3630 cannot run. |

## ⛔ Not importable (384)

No Pocket PC program inside the download (desktop tools, ROM updates, documents).

| Title | File | Result | Notes |
|---|---|---|---|
| [[REDUMP] Handheld - Microsoft - Pocket PC - Datfile (82) (-2) (2026-10-14 05-51-55)](https://archive.org/details/pocket-pc-redump) | `Pocket Flash Format (Japan).zip` | Not importable | No Pocket PC program was found in it. |
| [[REDUMP] Handheld - Microsoft - Pocket PC - Datfile (82) (-2) (2026-10-14 05-51-55)](https://archive.org/details/pocket-pc-redump) | `Pocket Zipper (Japan).zip` | Not importable | No Pocket PC program was found in it. |
| [A few Games, Software and Themes for PocketPC](https://archive.org/details/game_20260818) | `Themes.rar` | Not importable | No Pocket PC program was found in it. |
| [A4News](https://archive.org/details/tucows_368497_A4News) | `A4NewsV1.zip` | Not importable | No Pocket PC program was found in it. |
| [abcDB 'Pro-Print' Database](https://archive.org/details/tucows_373902_abcDB_Pro-Print_Database) | `abcpropr6.exe` | Not importable | This is a desktop Windows program, and no Pocket PC game could be taken out of it. |
| [AC3 Codec for TCPMP 0.71 (v0.71) (Windows Mobile Port)](https://archive.org/details/ac3.ARM_SP) | `ac3.ARM_SP.CAB` | Not importable | No Pocket PC program was found in it. |
| [AccountLogon](https://archive.org/details/tucows_212625_AccountLogon) | `acclogon.exe` | Not importable | This is a desktop Windows program, and no Pocket PC game could be taken out of it. |
| [ACDSee Mobile](https://archive.org/details/tucows_217585_ACDSee_Mobile) | `acdseemobile-wce.exe` | Not importable | This is a desktop Windows program, and no Pocket PC game could be taken out of it. |
| [Acer beTouch E200 Companion Discs](https://archive.org/details/acer-betouch-e200-companion-discs) | `Acer beTouch E200 - UMU Mobile Security Suite.zip` | Not importable | No Pocket PC program was found in it. |
| [Acer N35 Firmware](https://archive.org/details/acer-n35-firmware) | `n35_SE_Eboot_6.2.rar` | Not importable | No Pocket PC program was found in it. |
| [Acer N35 Firmware](https://archive.org/details/acer-n35-firmware) | `n35_sdv1.40.32_wwe.zip` | Not importable | No Pocket PC program was found in it. |
| [AchieveIT Goal Setter](https://archive.org/details/tucows_318170_AchieveIT_Goal_Setter) | `AchieveIT.exe` | Not importable |  |
| [ActiveSite Compiler](https://archive.org/details/tucows_348772_ActiveSite_Compiler) | `sitecomp.exe` | Not importable | This is a desktop Windows program, and no Pocket PC game could be taken out of it. |
| [Adobe Reader for Pocket PC v2.0](https://archive.org/details/Adobe_ReaderPPC-2) | `AdbeRdr20_ppc_enu.exe` | Not importable | This is a desktop Windows program, and no Pocket PC game could be taken out of it. |
| [Advanced Merchant Solutions Pocket Verifier Lite](https://archive.org/details/tucows_200471_Advanced_Merchant_Solutions_Pocket_Verifier_Lite) | `pvce.exe` | Not importable | This is a desktop Windows program, and no Pocket PC game could be taken out of it. |
| [Ariadne Organizer](https://archive.org/details/tucows_352128_Ariadne_Organizer) | `ariadnePPC.exe` | Not importable | This is a desktop Windows program, and no Pocket PC game could be taken out of it. |
| [ATmini Plus](https://archive.org/details/tucows_51463_ATmini_Plus) | `Atm12.zip` | Not importable | No Pocket PC program was found in it. |
| [BeamLink](https://archive.org/details/tucows_32187_BeamLink) | `BLSetup-1.01.exe` | Not importable | This is a desktop Windows program, and no Pocket PC game could be taken out of it. |
| [BeamWorks](https://archive.org/details/tucows_70893_BeamWorks) | `beamworks10_eval.zip` | Not importable | No Pocket PC program was found in it. |
| [BirthdayBoy](https://archive.org/details/tucows_184085_BirthdayBoy) | `BBoyTrialSetup.exe` | Not importable | This is a desktop Windows program, and no Pocket PC game could be taken out of it. |
| [Blocks](https://archive.org/details/tucows_240055_Blocks) | `blocks.zip` | Not importable | No Pocket PC program was found in it. |
| [Boomeran](https://archive.org/details/tucows_32465_Boomeran) | `bmr10sh3.exe` | Not importable | This is a desktop Windows program, and no Pocket PC game could be taken out of it. |
| [Bryht Flash Player](https://archive.org/details/tucows_344078_Bryht_Flash_Player) | `BryhtFlashV20.exe` | Not importable | This is a desktop Windows program, and no Pocket PC game could be taken out of it. |
| [Bryht FlashApp Builder](https://archive.org/details/tucows_373403_Bryht_FlashApp_Builder) | `FlashAppBuilderV10.exe` | Not importable | This is a desktop Windows program, and no Pocket PC game could be taken out of it. |
| [Bryht FlashApp Builder](https://archive.org/details/tucows_373404_Bryht_FlashApp_Builder) | `FlashAppBuilderWinV10.exe` | Not importable | This is a desktop Windows program, and no Pocket PC game could be taken out of it. |
| [BugLord 1.0](https://archive.org/details/buglord-1.0) | `buglord-pocketpc-1.00.exe` | Not importable | This is a desktop Windows program, and no Pocket PC game could be taken out of it. |
| [buglord-pocketpc-1.00](https://archive.org/details/buglord-pocketpc-1.00) | `buglord-pocketpc-1.00.exe` | Not importable | This is a desktop Windows program, and no Pocket PC game could be taken out of it. |
| [Bullant Remote](https://archive.org/details/tucows_280364_Bullant_Remote) | `BullantRemoteForPalmOS.zip` | Not importable | No Pocket PC program was found in it. |
| [Button Mapper](https://archive.org/details/tucows_227585_Button_Mapper) | `Buttonmap.zip` | Not importable | No Pocket PC program was found in it. |
| [C# IDE Mobile for Pocket PC (Windows CE)](https://archive.org/details/cs_ide_mobile) | `cs_ide_mobile_samples.zip` | Not importable | No Pocket PC program was found in it. |
| [C# IDE Mobile for Pocket PC (Windows CE)](https://archive.org/details/cs_ide_mobile) | `cs_ide_mobile.zip` | Not importable | No Pocket PC program was found in it. |
| [Calendae](https://archive.org/details/tucows_300615_Calendae) | `CalendaePPC.exe` | Not importable | This is a desktop Windows program, and no Pocket PC game could be taken out of it. |
| [Call of Duty 2 (USA)](https://archive.org/details/Nova_CoD2PPC_USA) | `[LOGS] Call of Duty 2 (USA).zip` | Not importable | No Pocket PC program was found in it. |
| [CalliGrapher](https://archive.org/details/tucows_32567_CalliGrapher) | `calligrapher60e.zip` | Not importable | This is not a Pocket PC CAB, or it is damaged. |
| [CardScan Executive](https://archive.org/details/tucows_32396_CardScan_Executive) | `cs5trial.exe` | Not importable | This is a desktop Windows program, and no Pocket PC game could be taken out of it. |
| [Cave Man](https://archive.org/details/tucows_32076_Cave_Man) | `CMDemo.exe` | Not importable | This is a desktop Windows program, and no Pocket PC game could be taken out of it. |
| [CE My Family](https://archive.org/details/tucows_295838_CE_My_Family) | `CEMFDist.exe` | Not importable | This is a desktop Windows program, and no Pocket PC game could be taken out of it. |
| [CE-Star](https://archive.org/details/tucows_32258_CE-Star) | `ce-starhpc.exe` | Not importable | This is a desktop Windows program, and no Pocket PC game could be taken out of it. |
| [CEBible](https://archive.org/details/tucows_32257_CEBible) | `CEBibleDemo.exe` | Not importable | This is a desktop Windows program, and no Pocket PC game could be taken out of it. |
| [CEMon](https://archive.org/details/tucows_32476_CEMon) | `CEFileMonitor.EXE` | Not importable | This is a desktop Windows program, and no Pocket PC game could be taken out of it. |
| [CEPlaylist](https://archive.org/details/tucows_264278_CEPlaylist) | `CEPlaylist.zip` | Not importable | No Pocket PC program was found in it. |
| [CESlideShow](https://archive.org/details/tucows_293255_CESlideShow) | `CESlideShow102.zip` | Not importable | No Pocket PC program was found in it. |
| [Championship Hearts](https://archive.org/details/tucows_264204_Championship_Hearts) | `InstallHeartsT0Palm.zip` | Not importable | No Pocket PC program was found in it. |
| [chm2web](https://archive.org/details/tucows_269862_chm2web) | `chm2web101.zip` | Not importable | No Pocket PC program was found in it. |
| [ChoiceWay Guide to Rome](https://archive.org/details/tucows_46513_ChoiceWay_Guide_to_Rome) | `rdemo.exe` | Not importable | This is a desktop Windows program, and no Pocket PC game could be taken out of it. |
| [Chopper Alley (USA)](https://archive.org/details/Nova_ChopperAlleyPPC_USA) | `[LOGS] Chopper Alley (USA).zip` | Not importable | No Pocket PC program was found in it. |
| [CitiKey London](https://archive.org/details/tucows_70109_CitiKey_London) | `citikeylondon.zip` | Not importable | No Pocket PC program was found in it. |
| [City Time](https://archive.org/details/tucows_199547_City_Time) | `CityTime_PPC.exe` | Not importable | This is a desktop Windows program, and no Pocket PC game could be taken out of it. |
| [ClearType Applet](https://archive.org/details/tucows_253429_ClearType_Applet) | `CT100PPE.EXE` | Not importable | This is a desktop Windows program, and no Pocket PC game could be taken out of it. |
| [CNetX Pocket Slideshow](https://archive.org/details/tucows_208810_CNetX_Pocket_Slideshow) | `ss130ppe.exe` | Not importable | This is a desktop Windows program, and no Pocket PC game could be taken out of it. |
| [CodeWallet Pro](https://archive.org/details/tucows_221053_CodeWallet_Pro) | `devonecodewalletprotrialsetup.exe` | Not importable | This is a desktop Windows program, and no Pocket PC game could be taken out of it. |
| [Commando (USA)](https://archive.org/details/Nova_CommandoPDA_USA) | `[LOGS] Commando (USA).zip` | Not importable | No Pocket PC program was found in it. |
| [CompanionLink Express](https://archive.org/details/tucows_32400_CompanionLink_Express) | `clxsetup.exe` | Not importable | This is a desktop Windows program, and no Pocket PC game could be taken out of it. |
| [Conduits Pocket Slides 2.51 (Extracted)](https://archive.org/details/conduits-pocket-slides-2.51) | `Presentation Pack 2.PPC300_2577.cab` | Not importable | No Pocket PC program was found in it. |
| [Conduits Pocket Slides 2.51 (Extracted)](https://archive.org/details/conduits-pocket-slides-2.51) | `Pocket Slides Languages.PPC300.cab` | Not importable | No Pocket PC program was found in it. |
| [CoolCalc](https://archive.org/details/tucows_152073_CoolCalc) | `CoolCalcPocketPC.exe` | Not importable | This is a desktop Windows program, and no Pocket PC game could be taken out of it. |
| [Crazy8](https://archive.org/details/tucows_269399_Crazy8) | `DemoCrazy82002.zip` | Not importable | This is not a Pocket PC CAB, or it is damaged. |
| [Cresotech PocketLANce](https://archive.org/details/tucows_228387_Cresotech_PocketLANce) | `pocketlance.exe` | Not importable | This is a desktop Windows program, and no Pocket PC game could be taken out of it. |
| [Cresotech PocketPoint](https://archive.org/details/tucows_170494_Cresotech_PocketPoint) | `pocketpoint.exe` | Not importable | This is a desktop Windows program, and no Pocket PC game could be taken out of it. |
| [Crime of the Millenia](https://archive.org/details/tucows_209353_Crime_of_the_Millenia) | `Millenia.zip` | Not importable | No Pocket PC program was found in it. |
| [Cruel](https://archive.org/details/tucows_32124_Cruel) | `cruel.exe` | Not importable | This is a desktop Windows program, and no Pocket PC game could be taken out of it. |
| [CVP Cost Volume Profit Calculator](https://archive.org/details/tucows_225422_CVP_Cost_Volume_Profit_Calculator) | `cvpsetup.zip` | Not importable | No Pocket PC program was found in it. |
| [Darxide EMP 1.78 (PocketPC)](https://archive.org/details/darxide_emp_ppc_general_1.78) | `darxide_emp_ppc_general_1.78.rar` | Not importable | No Pocket PC program was found in it. |
| [Data On The Run](https://archive.org/details/tucows_208663_Data_On_The_Run) | `dotr4.exe` | Not importable | This is a desktop Windows program, and no Pocket PC game could be taken out of it. |
| [DatacompressionCE ActiveX](https://archive.org/details/tucows_259111_DatacompressionCE_ActiveX) | `DatacompressionCE-trial.zip` | Not importable | No Pocket PC program was found in it. |
| [DBCE](https://archive.org/details/tucows_32336_DBCE) | `DBCE102m3.zip` | Not importable | No Pocket PC program was found in it. |
| [Decuma OnSpot for Pocket PC and Palm OS](https://archive.org/details/decuma-onspot-ppc-palmos) | `decuma.kg.rar` | Not importable | No Pocket PC program was found in it. |
| [Dell Axim X5 ROM Update for Pocket PC 2002, v.05200300EN, A02](https://archive.org/details/r-63937) | `French/R63979.EXE` | Not importable | This is a desktop Windows program, and no Pocket PC game could be taken out of it. |
| [Dell Axim X5 ROM Update for Pocket PC 2002, v.05200300EN, A02](https://archive.org/details/r-63937) | `Spanish/R62174.EXE` | Not importable | This is a desktop Windows program, and no Pocket PC game could be taken out of it. |
| [Dell Axim X51v ROM Pack](https://archive.org/details/x-51v-roms) | `a12wm5diad_k.zip` | Not importable | No Pocket PC program was found in it. |
| [Dell Axim X51v ROM Pack](https://archive.org/details/x-51v-roms) | `football_wm5_a17en.zip` | Not importable | No Pocket PC program was found in it. |
| [Dell Venue Pro drivers](https://archive.org/details/dvp-usb-driver) | `DVP-HTC_driver.zip` | Not importable | No Pocket PC program was found in it. |
| [Dell Venue Pro drivers](https://archive.org/details/dvp-usb-driver) | `DVP_USB_Driver.zip` | Not importable | No Pocket PC program was found in it. |
| [Desktop Diet](https://archive.org/details/tucows_330275_Desktop_Diet) | `ddietsa.exe` | Not importable | This is a desktop Windows program, and no Pocket PC game could be taken out of it. |
| [DialFix](https://archive.org/details/tucows_32552_DialFix) | `dialfix_mips.zip` | Not importable | No Pocket PC program was found in it. |
| [DIGIVUE](https://archive.org/details/tucows_340125_DIGIVUE) | `bdvdh_en.zip` | Not importable | No Pocket PC program was found in it. |
| [Doom ( Pocket PC)](https://archive.org/details/doom-for-pocket-pc) | `Doom (PocketPC).zip` | Not importable | No Pocket PC program was found in it. |
| [dotNET Compact Framework 1.0 SP3 Redistributable (Extracted)](https://archive.org/details/NETCFv1SP3) | `netcf.core.ppc3.ARM.cab` | Not importable | This is not a Pocket PC CAB, or it is damaged. |
| [dotNET Compact Framework 1.0 SP3 Redistributable (Extracted)](https://archive.org/details/NETCFv1SP3) | `netcf.all.wce4.ARMV4.cab` | Not importable | This is not a Pocket PC CAB, or it is damaged. |
| [Drivers for Nexian NXC3100 NexiCam Digital Camera sleeve for HP iPaq Pocket PC](https://archive.org/details/nexicam-ipaq-drivers) | `NexiCam/autoplay.exe` | Not importable | This is a desktop Windows program, and no Pocket PC game could be taken out of it. |
| [DrummerBoy](https://archive.org/details/tucows_145392_DrummerBoy) | `DrummerBoy_Demo.zip` | Not importable | No Pocket PC program was found in it. |
| [DStore](https://archive.org/details/tucows_32355_DStore) | `dstore.zip` | Not importable | No Pocket PC program was found in it. |
| [DVD-to-Pocket PC](https://archive.org/details/tucows_368931_DVD-to-Pocket_PC) | `DVD2PPC.zip` | Not importable | No Pocket PC program was found in it. |
| [Embrosix Pocket PC Browser ActiveX](https://archive.org/details/tucows_291372_Embrosix_Pocket_PC_Browser_ActiveX) | `Setup_Embrosix_1003.exe` | Not importable | This is a desktop Windows program, and no Pocket PC game could be taken out of it. |
| [Emperor's Mahjong (Dutch) cd-rom](https://archive.org/details/empmahjongnl) | `Mahjong_logs.zip` | Not importable | No Pocket PC program was found in it. |
| [Empire of the Undead (USA)](https://archive.org/details/Nova_EmpireOfTheUndead_USA) | `Empire of the Undead (USA)/[LOGS] Empire of the Undead (USA).zip` | Not importable | No Pocket PC program was found in it. |
| [Encryption Toolkit 2003](https://archive.org/details/tucows_301084_Encryption_Toolkit_2003) | `NET03TE.exe` | Not importable | This is a desktop Windows program, and no Pocket PC game could be taken out of it. |
| [Escape from Butcher Bay PocketPC theme](https://archive.org/details/EFBB-PocketPC) | `EFBB.zip` | Not importable | No Pocket PC program was found in it. |
| [EverQuest for the Pocket PC (USA)](https://archive.org/details/Nova_EQPPC_USA) | `[LOGS] EverQuest for the Pocket PC (USA).zip` | Not importable | No Pocket PC program was found in it. |
| [EZ-Expense](https://archive.org/details/tucows_300348_EZ-Expense) | `ezxptrial.exe` | Not importable | This is a desktop Windows program, and no Pocket PC game could be taken out of it. |
| [Fade (Version 1.09) [Pocket PC]](https://archive.org/details/fade-1.09-pocket-pc) | `Fade (1.09).rar` | Not importable | No Pocket PC program was found in it. |
| [ffmpeg for TCPMP v.0.71 (v0.71) (Windows Mobile Port)](https://archive.org/details/ffmpeg.ARM_SP) | `ffmpeg.ARM_SP.CAB` | Not importable | No Pocket PC program was found in it. |
| [FireFrontier](https://archive.org/details/tucows_222274_FireFrontier) | `FireFrontier_CASIOPPC_120EN_TR.exe` | Not importable | This is a desktop Windows program, and no Pocket PC game could be taken out of it. |
| [FIS WinCE Suite](https://archive.org/details/tucows_55862_FIS_WinCE_Suite) | `fisce200.zip` | Not importable | No Pocket PC program was found in it. |
| [Flash Player 7 for Pocket PC 2003/Windows CE 4.2](https://archive.org/details/fp7_ppc_en) | `fp7_ppc_en.cab` | Not importable | No Pocket PC program was found in it. |
| [Flashit](https://archive.org/details/tucows_201253_Flashit) | `flashit.zip` | Not importable | No Pocket PC program was found in it. |
| [Format Arabic .NET Class](https://archive.org/details/tucows_336981_Format_Arabic_.NET_Class) | `FArabic.zip` | Not importable | The archive is damaged or cannot be read. |
| [FormBuilder](https://archive.org/details/tucows_32358_FormBuilder) | `FormBuilderDemo.zip` | Not importable | No Pocket PC program was found in it. |
| [Four in a row](https://archive.org/details/tucows_275297_Four_in_a_row) | `4rowhpc.zip` | Not importable | No Pocket PC program was found in it. |
| [FREE MapFactor Navigator 23.0 Open Street Maps data version](https://archive.org/details/mapfactor23osm) | `Navigator23/Setup Utility/setup.exe` | Not importable | This is a desktop Windows program, and no Pocket PC game could be taken out of it. |
| [FREE MapFactor Navigator 23.0 Open Street Maps data version](https://archive.org/details/mapfactor23osm) | `Navigator23/Setup Utility/clickertray.exe` | Not importable | This is a desktop Windows program, and no Pocket PC game could be taken out of it. |
| [FreePascal Compiler 2.4.4 ARM Windows CE](https://archive.org/details/fpc-2.4.4.arm-wince) | `fpc-2.4.4.arm-wince.exe` | Not importable | This is a desktop Windows program, and no Pocket PC game could be taken out of it. |
| [Fun2Link](https://archive.org/details/tucows_184547_Fun2Link) | `fun2link_pocketpc_demo.exe` | Not importable | This is a desktop Windows program, and no Pocket PC game could be taken out of it. |
| [FunnySnake ThemeKing](https://archive.org/details/tucows_283724_FunnySnake_ThemeKing) | `ThemeKing.zip` | Not importable | No Pocket PC program was found in it. |
| [GenGisCad, (old ProCad)](https://archive.org/details/tucows_270961_GenGisCad_old_ProCad) | `gengiscadarm.zip` | Not importable | No Pocket PC program was found in it. |
| [Getting Started CD for the HP iPAQ hx2000 Pocket PC Series (USA)](https://archive.org/details/Nova_HPiPAQhx2000_USA) | `[LOGS] Getting Started CD for the HP iPAQ hx2000 Pocket PC Series (USA).zip` | Not importable | No Pocket PC program was found in it. |
| [Getting Started Disc for Windows Mobile 5.0 Upgrade (USA)](https://archive.org/details/Nova_WindowsMobile5Upgrade_USA) | `[LOGS] Getting Started Disc for Windows Mobile 5.0 Upgrade (USA).zip` | Not importable | No Pocket PC program was found in it. |
| [Graduate Chess (Europe) (En,Fr,De)](https://archive.org/details/Nova_GraduateChessPPC_EUR) | `[LOGS] Graduate Chess (Europe) (En,Fr,De).zip` | Not importable | No Pocket PC program was found in it. |
| [Griff](https://archive.org/details/tucows_243758_Griff) | `Griff_Setup.exe` | Not importable | This is a desktop Windows program, and no Pocket PC game could be taken out of it. |
| [Griff Music Sequencer for PocketPC](https://archive.org/details/griff-setup) | `MidiPlug_Setup.exe` | Not importable | This is a desktop Windows program, and no Pocket PC game could be taken out of it. |
| [GT-I8000 Getting Started Disc](https://archive.org/details/drvupdate-amd64_202609) | `GT-I8000/WMDC/WMDC_SR1_README_ALL_LOCALES.zip` | Not importable | No Pocket PC program was found in it. |
| [GT-I8000 Getting Started Disc](https://archive.org/details/drvupdate-amd64_202609) | `GT-I8000/SETUP/DHUninstall.exe` | Not importable | This is a desktop Windows program, and no Pocket PC game could be taken out of it. |
| [Haali Reader 2.0b227 - a book reader for Windows CE](https://archive.org/details/haali-reader-2.0b227-a-book-reader-for-windows-ce) | `EXTRAS/mkdict.exe` | Not importable | This is a desktop Windows program, and no Pocket PC game could be taken out of it. |
| [Haali Reader 2.0b264 for Pocket PC - a book reader for Windows CE](https://archive.org/details/20221104_20221104_0126) | `EXTRAS/mkdict.exe` | Not importable | This is a desktop Windows program, and no Pocket PC game could be taken out of it. |
| [HanDBase](https://archive.org/details/tucows_227257_HanDBase) | `handbaseppcdemo.exe` | Not importable | This is a desktop Windows program, and no Pocket PC game could be taken out of it. |
| [HandyZIP](https://archive.org/details/tucows_146753_HandyZIP) | `hz180hxe.exe` | Not importable | This is a desktop Windows program, and no Pocket PC game could be taken out of it. |
| [HeroCraft Misc Palm/Windows Mobile Titles](https://archive.org/details/install-robo-wm) | `robo2-editor.exe` | Not importable | This is a desktop Windows program, and no Pocket PC game could be taken out of it. |
| [Hexacto's Bob the Pipe Fitter](https://archive.org/details/tucows_229568_Hexacto_s_Bob_the_Pipe_Fitter) | `PipeFitterDemo_1.0.1_AllSetup.exe` | Not importable | This is a desktop Windows program, and no Pocket PC game could be taken out of it. |
| [Hexacto's Bounty Hunter 2099 Pinball](https://archive.org/details/tucows_244902_Hexacto_s_Bounty_Hunter_2099_Pinball) | `PinballDemo_1.1.1_AllSetup.exe` | Not importable | This is a desktop Windows program, and no Pocket PC game could be taken out of it. |
| [HP iPAQ Companion CD (USA) (hx2000 Series)](https://archive.org/details/Nova_HPiPAQCompanionHX2000_USA) | `[LOGS] HP iPAQ Companion CD (USA) (hx2000 Series).zip` | Not importable | No Pocket PC program was found in it. |
| [HP iPAQ hx2000 Pocket PC Software Disc](https://archive.org/details/wceusbsh_202609) | `OFFICE10/MS/ACTSYNC/Main/data1.cab` | Not importable | This is not a Pocket PC CAB, or it is damaged. |
| [HP iPAQ hx2000 Pocket PC Software Disc](https://archive.org/details/wceusbsh_202609) | `OFFICE10/MS/ACTSYNC/Main/_user1.cab` | Not importable | This is not a Pocket PC CAB, or it is damaged. |
| [HPC Notes Professional Edition](https://archive.org/details/tucows_219672_HPC_Notes_Professional_Edition) | `hpcnotespro.exe` | Not importable | This is a desktop Windows program, and no Pocket PC game could be taken out of it. |
| [HTC MAX 4G firmware and files collection](https://archive.org/details/htc-max-4g-collection) | `HSPL and SSPL/Restore_Stock_045_SPL.zip` | Not importable | No Pocket PC program was found in it. |
| [Images Control](https://archive.org/details/tucows_47716_Images_Control) | `IMCE101m3.zip` | Not importable | No Pocket PC program was found in it. |
| [Incesoft AnySame](https://archive.org/details/tucows_317149_Incesoft_AnySame) | `AnySame_Setup.zip` | Not importable | No Pocket PC program was found in it. |
| [Incesoft AnySame Express](https://archive.org/details/tucows_313611_Incesoft_AnySame_Express) | `AnySameEx.zip` | Not importable | No Pocket PC program was found in it. |
| [Infectious Ground](https://archive.org/details/tucows_232276_Infectious_Ground) | `IG11.1_iPAQ_2000x.zip` | Not importable | This is not a Pocket PC CAB, or it is damaged. |
| [Infectious Ground](https://archive.org/details/tucows_232276_Infectious_Ground) | `IG1.1_2002.zip` | Not importable | This is not a Pocket PC CAB, or it is damaged. |
| [InfoSafe Plus](https://archive.org/details/tucows_336123_InfoSafe_Plus) | `infosafeplus-setup.exe` | Not importable | This is a desktop Windows program, and no Pocket PC game could be taken out of it. |
| [IntelliGolf Birdie edition / Smartphones](https://archive.org/details/tucows_150724_IntelliGolf_Birdie_edition_Smartphones) | `igtbrdsp.exe` | Not importable | This is a desktop Windows program, and no Pocket PC game could be taken out of it. |
| [IntelliGolf for the Visor](https://archive.org/details/tucows_57153_IntelliGolf_for_the_Visor) | `intelligolfvisor.exe` | Not importable | This is a desktop Windows program, and no Pocket PC game could be taken out of it. |
| [iPAQ Pocket PC 2002 Service Pack 4](https://archive.org/details/sp26041) | `sp26041.exe` | Not importable | This is a desktop Windows program, and no Pocket PC game could be taken out of it. |
| [IProFalcon](https://archive.org/details/tucows_182043_IProFalcon) | `IProFalconSetup.exe` | Not importable | This is a desktop Windows program, and no Pocket PC game could be taken out of it. |
| [IrPass](https://archive.org/details/tucows_230828_IrPass) | `IrPassDemo.exe` | Not importable | This is a desktop Windows program, and no Pocket PC game could be taken out of it. |
| [JetForm Filler](https://archive.org/details/tucows_32362_JetForm_Filler) | `fillerce25demo.zip` | Not importable | No Pocket PC program was found in it. |
| [JimmyARK](https://archive.org/details/tucows_222305_JimmyARK) | `DemoArk2iPaqSetup.exe` | Not importable | This is a desktop Windows program, and no Pocket PC game could be taken out of it. |
| [JMPocket](https://archive.org/details/tucows_274963_JMPocket) | `jmpocket-ppc-1.0.1.zip` | Not importable | No Pocket PC program was found in it. |
| [Just a small clicker (Windows Mobile 6)](https://archive.org/details/smart-device-project-1) | `SmartDeviceProject1.zip` | Not importable | No Pocket PC program was found in it. |
| [Just a small clicker (Windows Mobile 6)](https://archive.org/details/smart-device1337) | `SmartDeviceProject1.zip` | Not importable | No Pocket PC program was found in it. |
| [KeyStrokeCE](https://archive.org/details/tucows_68152_KeyStrokeCE) | `keystrokece.zip` | Not importable | No Pocket PC program was found in it. |
| [KeyStrokeCE (Extracted)](https://archive.org/details/tucows_68152_KeyStrokeCE_extracted) | `KeyStrokeCE/KeyStrokeCE.ARM_PPC.cab` | Not importable | No Pocket PC program was found in it. |
| [KSE Abacus XT - English/German](https://archive.org/details/tucows_223201_KSE_Abacus_XT_-_English_German) | `abacuspocketpcenggermtrial.exe` | Not importable | This is a desktop Windows program, and no Pocket PC game could be taken out of it. |
| [Lazarus IDE 0.9.30 with FreePascalCompiler 2.4.2 Cross-ARM Windows CE x86](https://archive.org/details/lazarus-0.9.30-fpc-2.4.2-cross-arm-wince-win-32) | `Lazarus-0.9.30-fpc-2.4.2-cross-arm-wince-win32.exe` | Not importable | This is a desktop Windows program, and no Pocket PC game could be taken out of it. |
| [Lazarus IDE 1.6.2 with FreePascal Compiler 3.0.0 Cross-ARM Windows CE x86](https://archive.org/details/lazarus-1.6.2-fpc-3.0.0-cross-arm-wince-win32) | `lazarus-1.6.2-fpc-3.0.0-cross-arm-wince-win32.exe` | Not importable | This is a desktop Windows program, and no Pocket PC game could be taken out of it. |
| [Legendary Wings (USA)](https://archive.org/details/Nova_LegendaryWingsPDA_USA) | `[LOGS] Legendary Wings (USA).zip` | Not importable | No Pocket PC program was found in it. |
| [LHA](https://archive.org/details/tucows_32451_LHA) | `lha255e.exe` | Not importable | This is a desktop Windows program, and no Pocket PC game could be taken out of it. |
| [Links - Pocket PC Edition (USA)](https://archive.org/details/Nova_LinksPPC_USA) | `Links - Pocket PC Edition (USA)/[LOGS] Links - Pocket PC Edition (USA).zip` | Not importable | No Pocket PC program was found in it. |
| [LINKS For Pocket PC](https://archive.org/details/links-for-pocket-pc) | `links_ppc.exe` | Not importable | This is a desktop Windows program, and no Pocket PC game could be taken out of it. |
| [LoanExpert Plus](https://archive.org/details/tucows_309361_LoanExpert_Plus) | `loanexpertplus-setup.exe` | Not importable | This is a desktop Windows program, and no Pocket PC game could be taken out of it. |
| [LoanExpert Plus](https://archive.org/details/tucows_341932_LoanExpert_Plus) | `loanexpertplussetup.exe` | Not importable | This is a desktop Windows program, and no Pocket PC game could be taken out of it. |
| [Low Power Ethernet](https://archive.org/details/tucows_32203_Low_Power_Ethernet) | `lpece30.exe` | Not importable | This is a desktop Windows program, and no Pocket PC game could be taken out of it. |
| [Macromedia Flash Player 6 for Pocket PC 2002/Windows CE 3.0](https://archive.org/details/fp6activexce) | `activex.cab` | Not importable | No Pocket PC program was found in it. |
| [Mahjong (Mindscape, Dutch) cd-rom](https://archive.org/details/ms-mahjong) | `Mahjong_logs.zip` | Not importable | No Pocket PC program was found in it. |
| [MapFactor Free 21 Windows CE and Windows NT](https://archive.org/details/mapfactorfree21) | `NavigatorFree_install.exe` | Not importable | This is a desktop Windows program, and no Pocket PC game could be taken out of it. |
| [MapFactor Navigator Free 25 (OpenStreetMap Maps)](https://archive.org/details/mapfactor-navigator-free-25) | `Navigator25/Setup Utility/setup.exe` | Not importable | This is a desktop Windows program, and no Pocket PC game could be taken out of it. |
| [MapFactor Navigator Free 25 (OpenStreetMap Maps)](https://archive.org/details/mapfactor-navigator-free-25) | `Navigator25/Setup Utility/clickertray.exe` | Not importable | This is a desktop Windows program, and no Pocket PC game could be taken out of it. |
| [Meedio HouseBot](https://archive.org/details/tucows_373884_Meedio_HouseBot) | `MeedioHouseBot.2.20.setup.exe` | Not importable | This is a desktop Windows program, and no Pocket PC game could be taken out of it. |
| [Mega Touch 2 Hard SPL Kit By Jocky 2001](https://archive.org/details/megatouch2hardsplkitbyjocky2001) | `Mega-Touch2-HardSPL-Kit-by-jocky2001.zip` | Not importable | No Pocket PC program was found in it. |
| [Merlin Pocket PC Registry Editor](https://archive.org/details/tucows_266182_Merlin_Pocket_PC_Registry_Editor) | `TrialRegEdit.zip` | Not importable | No Pocket PC program was found in it. |
| [Merlin Pocket PC Tracert (and Ping)](https://archive.org/details/tucows_268776_Merlin_Pocket_PC_Tracert_and_Ping) | `TrialTracert.zip` | Not importable | No Pocket PC program was found in it. |
| [Message Master](https://archive.org/details/tucows_61073_Message_Master) | `mmforce.exe` | Not importable | This is a desktop Windows program, and no Pocket PC game could be taken out of it. |
| [Metalion (USA)](https://archive.org/details/Nova_Metalion_USA) | `Metalion (USA)/[LOGS] Metalion (USA).zip` | Not importable | No Pocket PC program was found in it. |
| [Metarim Pocket Salewiz](https://archive.org/details/tucows_271281_Metarim_Pocket_Salewiz) | `SaleWizSetup.exe` | Not importable | This is a desktop Windows program, and no Pocket PC game could be taken out of it. |
| [Microsoft Entertainment PocketPak (USA)](https://archive.org/details/Nova_MicrosoftPocketPak_USA) | `[LOGS] Microsoft Entertainment PocketPak (USA).zip` | Not importable | No Pocket PC program was found in it. |
| [Microsoft Expedia Streets 98 (USA)](https://archive.org/details/Nova_MicrosoftExpediaStreets98_USA) | `[LOGS] Microsoft Expedia Streets 98 (USA).zip` | Not importable | No Pocket PC program was found in it. |
| [Microsoft Fun Pocket PC Expansion Pack (USA)](https://archive.org/details/Nova_MSFunPocketPCExpansionCD_USA) | `[LOGS] Microsoft Fun Pocket PC Expansion Pack (USA).zip` | Not importable | No Pocket PC program was found in it. |
| [Microsoft Productivity Pack (USA)](https://archive.org/details/Nova_MicrosoftProductivityPackWinMobile_USA) | `[LOGS] Microsoft Productivity Pack (USA).zip` | Not importable | No Pocket PC program was found in it. |
| [Microsoft Streets and Trips 2003 (PC/Pocket PC) - Disc Dumps and Scans](https://archive.org/details/microsoft-streets-and-trips-2003) | `CD1/CD1_logs.zip` | Not importable | No Pocket PC program was found in it. |
| [Microsoft Streets and Trips 2003 (PC/Pocket PC) - Disc Dumps and Scans](https://archive.org/details/microsoft-streets-and-trips-2003) | `GEO10NA/GEO10NA_logs.zip` | Not importable | No Pocket PC program was found in it. |
| [Microsoft Windows 10 Mobile (Some builds)](https://archive.org/details/ms-win10mob-builds-01) | `10551.0.th2_release.150917-1700/microsoft.wehfm.spkr_7cdf2ba6241b08c2313350b25461b0ac487ad589.cab` | Not importable | No Pocket PC program was found in it. |
| [Microsoft Windows 10 Mobile (Some builds)](https://archive.org/details/ms-win10mob-builds-01) | `10551.0.th2_release.150917-1700/microsoft.ms_weh_usb.weh.mainos.spkr_c907de35114499d11352ab71eb0041eb2ba9c4a0.cab` | Not importable | No Pocket PC program was found in it. |
| [Microsoft Windows 10 Mobile for Xiaomi Mi4 LTE - 10586.1102.3063](https://archive.org/details/ms-w10m-xiaomi-mi4lte-10586-1102) | `MiFlash20150601.zip` | Not importable | No Pocket PC program was found in it. |
| [Microsoft Windows CE 3.00 - preinstalled VMDK image for VMware Workstation virtual machine](https://archive.org/details/windows-ce-3.00-vmdk) | `Windows CE 3.00.zip` | Not importable | No Pocket PC program was found in it. |
| [Midway Classics - Marble Madness (USA)](https://archive.org/details/Nova_MarbleMadnessPPC_USA) | `Midway Classics - Marble Madness (USA)/[LOGS] Midway Classics - Marble Madness (USA).zip` | Not importable | No Pocket PC program was found in it. |
| [Mineseeker](https://archive.org/details/tucows_78004_Mineseeker) | `hpcmines.zip` | Not importable | No Pocket PC program was found in it. |
| [Mobile PhotoPleasure](https://archive.org/details/tucows_349952_Mobile_PhotoPleasure) | `mpp.exe` | Not importable | This is a desktop Windows program, and no Pocket PC game could be taken out of it. |
| [MobiLearn OutLoud Spanish Health Care Phrases](https://archive.org/details/tucows_259120_MobiLearn_OutLoud_Spanish_Health_Care_Phrases) | `taberv2.0_trial.exe` | Not importable | This is a desktop Windows program, and no Pocket PC game could be taken out of it. |
| [Mobyware Pocket PC](https://archive.org/details/Mobyware-Pocket-PC) | `MobyPPC-C-2500.zip` | Not importable | No Pocket PC program was found in it. |
| [Mocha CE Telnet](https://archive.org/details/tucows_209731_Mocha_CE_Telnet) | `MochaCE.zip` | Not importable | No Pocket PC program was found in it. |
| [Mocha CE TN3270](https://archive.org/details/tucows_210297_Mocha_CE_TN3270) | `pock3270.zip` | Not importable | No Pocket PC program was found in it. |
| [Mocha Telnet](https://archive.org/details/tucows_209758_Mocha_Telnet) | `hpc211te.zip` | Not importable | No Pocket PC program was found in it. |
| [Mocha TN3270](https://archive.org/details/tucows_210300_Mocha_TN3270) | `hpc21132.zip` | Not importable | No Pocket PC program was found in it. |
| [Mocha TN5250](https://archive.org/details/tucows_71850_Mocha_TN5250) | `hpc5250.zip` | Not importable | No Pocket PC program was found in it. |
| [MorphGear 2.4.0.9 HD2 PPC2003](https://archive.org/details/morphgear2409-hd2-xdaforums) | `HuGo/HuGoPPC2003.ARM.cab` | Not importable | No Pocket PC program was found in it. |
| [MorphGear 2.4.0.9 HD2 PPC2003](https://archive.org/details/morphgear2409-hd2-xdaforums) | `SNES9x/SNES9xPPC2003.ARM.cab` | Not importable | No Pocket PC program was found in it. |
| [Motocross Stunt Racer (USA)](https://archive.org/details/Nova_MotocrossStuntRacerPPC_USA) | `[LOGS] Motocross Stunt Racer (USA).zip` | Not importable | No Pocket PC program was found in it. |
| [Motorola MPx200 2006v1 Custom ROM](https://archive.org/details/mpx200_2006v1) | `2006v1.rar` | Not importable | No Pocket PC program was found in it. |
| [Motorola MPx200 WM 2005v2](https://archive.org/details/wm2005v2) | `WM2005v2.rar` | Not importable | No Pocket PC program was found in it. |
| [Movie2MemCard](https://archive.org/details/tucows_355307_Movie2MemCard) | `m2m12trial.exe` | Not importable | This is a desktop Windows program, and no Pocket PC game could be taken out of it. |
| [MovieCub](https://archive.org/details/tucows_145780_MovieCub) | `moviecub.exe` | Not importable | This is a desktop Windows program, and no Pocket PC game could be taken out of it. |
| [MPx200 WM6 RUS.v2 RC3E by Strik3r](https://archive.org/details/mpx200.wm6.rus.v2rc3e.by.strik3r) | `WM6.RUS.v2RC3E.by.Strik3r.zip` | Not importable | No Pocket PC program was found in it. |
| [MPx220 Firmware 1.49](https://archive.org/details/WidowsMobile_for_MPx220__mpx220_1.49) | `motorola_mpx220_1.49-794.zip` | Not importable | The archive is damaged or cannot be read. |
| [MPx220 Firmware 1.53 MPXLand](https://archive.org/details/mpx-220-rom-1-53-mpxland) | `MPx220_ROM_1_53_MPXLand.zip` | Not importable | No Pocket PC program was found in it. |
| [MuseViewer](https://archive.org/details/tucows_276831_MuseViewer) | `MVP33ins.exe` | Not importable | This is a desktop Windows program, and no Pocket PC game could be taken out of it. |
| [MyMobiler 1.2.5 for Windows Mobile](https://archive.org/details/MyMobiler) | `MyMobiler_WM_1.2.5.exe` | Not importable | This is a desktop Windows program, and no Pocket PC game could be taken out of it. |
| [MySettings](https://archive.org/details/tucows_305880_MySettings) | `mysettings.zip` | Not importable | No Pocket PC program was found in it. |
| [News Force](https://archive.org/details/tucows_32209_News_Force) | `NewsForcehpc.exe` | Not importable | This is a desktop Windows program, and no Pocket PC game could be taken out of it. |
| [NFS Undercover - Windows Mobile (Qualcomm) Port (Android/Windows)](https://archive.org/details/wm_uc_port) | `Optimized.zip` | Not importable | No Pocket PC program was found in it. |
| [NFS Undercover - Windows Mobile (Qualcomm) Port (Android/Windows)](https://archive.org/details/wm_uc_port) | `WM.zip` | Not importable | No Pocket PC program was found in it. |
| [ObjectStore PSE](https://archive.org/details/tucows_32367_ObjectStore_PSE) | `cebeta.zip` | Not importable | No Pocket PC program was found in it. |
| [Office Mobile 6.1 and OneNote Mobile](https://archive.org/details/office-mobile-and-onenote-mobile) | `OneNoteMobile.rar` | Not importable | No Pocket PC program was found in it. |
| [Ookii Japanese Input](https://archive.org/details/japaneseinputookii) | `JapaneseInputSrc.zip` | Not importable | No Pocket PC program was found in it. |
| [Ookii Japanese Input](https://archive.org/details/japaneseinputookii) | `JapaneseInputWM2003.CAB` | Not importable | No Pocket PC program was found in it. |
| [OPClip](https://archive.org/details/tucows_374551_OPClip) | `OPClip.exe` | Not importable | This is a desktop Windows program, and no Pocket PC game could be taken out of it. |
| [Other OEM Windows Phone ROM Archive](https://archive.org/details/wpdb_archive) | `Tools/Samsung/WP8 SMD binary USB Downloader Ver. 3.47.exe` | Not importable | This is a desktop Windows program, and no Pocket PC game could be taken out of it. |
| [P-CIM](https://archive.org/details/tucows_32368_P-CIM) | `afconmpsinst.exe` | Not importable | This is a desktop Windows program, and no Pocket PC game could be taken out of it. |
| [PDAMedia Light Keyboard](https://archive.org/details/PDAMediaLightKeyboard) | `SetupSipKbdLight10/install.PPC_ARM.CAB` | Not importable | No Pocket PC program was found in it. |
| [PDAMedia Light Keyboard](https://archive.org/details/PDAMediaLightKeyboard) | `SetupSipKbdLight10/install.PPC_ARMF.CAB` | Not importable | No Pocket PC program was found in it. |
| [PE Net Explorer 2002](https://archive.org/details/tucows_184155_PE_Net_Explorer_2002) | `penet2002_61.zip` | Not importable | No Pocket PC program was found in it. |
| [PE Pocket Explorer 2002](https://archive.org/details/tucows_192463_PE_Pocket_Explorer_2002) | `pepocket2002_61.zip` | Not importable | No Pocket PC program was found in it. |
| [Pencil Box Deluxe](https://archive.org/details/tucows_351566_Pencil_Box_Deluxe) | `PencilBoxDeluxe.exe` | Not importable | This is a desktop Windows program, and no Pocket PC game could be taken out of it. |
| [Phonature Music Visualizer (PhonViso)](https://archive.org/details/tucows_311311_Phonature_Music_Visualizer_PhonViso) | `PhonViso.zip` | Not importable | No Pocket PC program was found in it. |
| [PicturePerfect](https://archive.org/details/tucows_149414_PicturePerfect) | `PicturePerfectSetup.exe` | Not importable | This is a desktop Windows program, and no Pocket PC game could be taken out of it. |
| [Playmax for Baseball](https://archive.org/details/tucows_228739_Playmax_for_Baseball) | `pmxbppcd.exe` | Not importable | This is a desktop Windows program, and no Pocket PC game could be taken out of it. |
| [Playmax for Basketball](https://archive.org/details/tucows_228737_Playmax_for_Basketball) | `pmxkppcd.exe` | Not importable | This is a desktop Windows program, and no Pocket PC game could be taken out of it. |
| [Playmax for FieldHockey](https://archive.org/details/tucows_228736_Playmax_for_FieldHockey) | `pmxyppcd.exe` | Not importable | This is a desktop Windows program, and no Pocket PC game could be taken out of it. |
| [Playmax for Football](https://archive.org/details/tucows_228731_Playmax_for_Football) | `pmxfppcd.exe` | Not importable | This is a desktop Windows program, and no Pocket PC game could be taken out of it. |
| [Playmax for Hockey](https://archive.org/details/tucows_228732_Playmax_for_Hockey) | `pmxhppcd.exe` | Not importable | This is a desktop Windows program, and no Pocket PC game could be taken out of it. |
| [Playmax for Lacrosse](https://archive.org/details/tucows_228735_Playmax_for_Lacrosse) | `pmxlppcd.exe` | Not importable | This is a desktop Windows program, and no Pocket PC game could be taken out of it. |
| [Playmax for Soccer](https://archive.org/details/tucows_228733_Playmax_for_Soccer) | `pmxsppcd.exe` | Not importable | This is a desktop Windows program, and no Pocket PC game could be taken out of it. |
| [Playmax for Volleyball](https://archive.org/details/tucows_228738_Playmax_for_Volleyball) | `pmxvppcd.exe` | Not importable | This is a desktop Windows program, and no Pocket PC game could be taken out of it. |
| [Pocket Athlete (USA)](https://archive.org/details/Nova_PocketAthletePPC_USA) | `[LOGS] Pocket Athlete (USA).zip` | Not importable | No Pocket PC program was found in it. |
| [Pocket AutoGraph](https://archive.org/details/tucows_221087_Pocket_AutoGraph) | `devonepocketautographtrialsetup.exe` | Not importable | This is a desktop Windows program, and no Pocket PC game could be taken out of it. |
| [Pocket Bass Pro (USA)](https://archive.org/details/Nova_PocketBassProPPC_USA) | `[LOGS] Pocket Bass Pro (USA).zip` | Not importable | No Pocket PC program was found in it. |
| [Pocket BizCalc](https://archive.org/details/tucows_32320_Pocket_BizCalc) | `pbcdemo.exe` | Not importable | This is a desktop Windows program, and no Pocket PC game could be taken out of it. |
| [Pocket CHM Professional](https://archive.org/details/tucows_364272_Pocket_CHM_Professional) | `pchmpro.exe` | Not importable | This is a desktop Windows program, and no Pocket PC game could be taken out of it. |
| [Pocket Cook Deluxe Web Edition](https://archive.org/details/tucows_151506_Pocket_Cook_Deluxe_Web_Edition) | `pcdwe.exe` | Not importable | This is a desktop Windows program, and no Pocket PC game could be taken out of it. |
| [Pocket CT-ART (Russia) (En,Fr,De,Es,It,Ru)](https://archive.org/details/Nova_PocketCTART_Russia) | `Pocket CT-ART (Russia) (En,Fr,De,Es,It,Ru)/[LOGS] Pocket CT-ART (Russia) (En,Fr,De,Es,It,Ru).zip` | Not importable | No Pocket PC program was found in it. |
| [Pocket Family Researcher](https://archive.org/details/tucows_60082_Pocket_Family_Researcher) | `PFR.zip` | Not importable | No Pocket PC program was found in it. |
| [Pocket Fritz (Europe) (En,Fr,De,Es,It,Nl,Sv)](https://archive.org/details/Nova_PocketFritz_Europe) | `Pocket Fritz (Europe) (En,Fr,De,Es,It,Nl,Sv)/[LOGS] Pocket Fritz (Europe) (En,Fr,De,Es,It,Nl,Sv).zip` | Not importable | No Pocket PC program was found in it. |
| [Pocket Gambler (USA)](https://archive.org/details/Nova_PocketGambler_USA) | `Pocket Gambler (USA)/[LOGS] Pocket Gambler (USA).zip` | Not importable | No Pocket PC program was found in it. |
| [Pocket Lookout 2](https://archive.org/details/tucows_212877_Pocket_Lookout_2) | `lookout2_setup.zip` | Not importable | No Pocket PC program was found in it. |
| [Pocket Maple Reader](https://archive.org/details/tucows_339417_Pocket_Maple_Reader) | `pmreader.exe` | Not importable | This is a desktop Windows program, and no Pocket PC game could be taken out of it. |
| [Pocket Marbles](https://archive.org/details/tucows_222081_Pocket_Marbles) | `trial_devonepocketmarblessetup.exe` | Not importable | This is a desktop Windows program, and no Pocket PC game could be taken out of it. |
| [Pocket PC - Redump.info](https://archive.org/details/pocket_pc) | `Pocket Zipper (Japan).zip` | Not importable | No Pocket PC program was found in it. |
| [Pocket PC 2003 Upgrade for Compaq iPAQ 3970](https://archive.org/details/3970upgrade_202108) | `romupdate.exe` | Not importable | This is a desktop Windows program, and no Pocket PC game could be taken out of it. |
| [Pocket PC and Windows Mobile/CE software, roms and mixed stuff.](https://archive.org/details/packs_202105) | `SpbFinance2.arm.cab` | Not importable | This is not a Pocket PC CAB, or it is damaged. |
| [Pocket PC Calculator](https://archive.org/details/tucows_362926_Pocket_PC_Calculator) | `Antcom-MathCal-Trial-v1.exe` | Not importable | This is a desktop Windows program, and no Pocket PC game could be taken out of it. |
| [Pocket Pc Emulator native frames per second hack](https://archive.org/details/pocket_pc_emulator) | `DeviceEmulator.exe` | Not importable | This is a desktop Windows program, and no Pocket PC game could be taken out of it. |
| [Pocket PC Install Builder](https://archive.org/details/tucows_272289_Pocket_PC_Install_Builder) | `PPCInstallBuilder.zip` | Not importable | This is not a Pocket PC CAB, or it is damaged. |
| [Pocket PC StarterPak for Dummies (USA)](https://archive.org/details/Nova_PPCForDummies_USA) | `[LOGS] Pocket PC StarterPak for Dummies (USA).zip` | Not importable | No Pocket PC program was found in it. |
| [Pocket ScreenSnap](https://archive.org/details/tucows_222133_Pocket_ScreenSnap) | `devonepocketscreensnaptrialsetup.exe` | Not importable | This is a desktop Windows program, and no Pocket PC game could be taken out of it. |
| [Pocket VirtualBoss](https://archive.org/details/tucows_331223_Pocket_VirtualBoss) | `pvbsetup.exe` | Not importable | This is a desktop Windows program, and no Pocket PC game could be taken out of it. |
| [PocketDrums](https://archive.org/details/tucows_236279_PocketDrums) | `PocketDrums_Demo.zip` | Not importable | No Pocket PC program was found in it. |
| [PocketGIS](https://archive.org/details/tucows_70787_PocketGIS) | `PocketGISDemo.exe` | Not importable | This is a desktop Windows program, and no Pocket PC game could be taken out of it. |
| [PocketPics](https://archive.org/details/tucows_182723_PocketPics) | `pocketpics1.zip` | Not importable | No Pocket PC program was found in it. |
| [PocketSat+](https://archive.org/details/tucows_308592_PocketSat) | `PocketSatPlus_2k2.zip` | Not importable | No Pocket PC program was found in it. |
| [PocketSketch](https://archive.org/details/tucows_200793_PocketSketch) | `chibiinste.exe` | Not importable | This is a desktop Windows program, and no Pocket PC game could be taken out of it. |
| [PocketSynth Pro](https://archive.org/details/tucows_199929_PocketSynth_Pro) | `PocketSynth_Professional_Demo.zip` | Not importable | No Pocket PC program was found in it. |
| [PocketWavPlayer](https://archive.org/details/tucows_220875_PocketWavPlayer) | `VoiceRecorderSampleClient.zip` | Not importable | No Pocket PC program was found in it. |
| [PocketXchange](https://archive.org/details/tucows_366065_PocketXchange) | `PocketXchange-ver1.zip` | Not importable | No Pocket PC program was found in it. |
| [PointStart Today](https://archive.org/details/tucows_222137_PointStart_Today) | `trial_devonepointstarttodaysetup.exe` | Not importable | This is a desktop Windows program, and no Pocket PC game could be taken out of it. |
| [PostPet for Windows CE (Japan)](https://archive.org/details/Nova_PostPetWinCE_JPN) | `[LOGS] PostPet for Windows CE (Japan).zip` | Not importable | No Pocket PC program was found in it. |
| [PowerBuyer 6.5 (Japan)](https://archive.org/details/Nova_PowerBuyer65_JPN) | `[LOGS] PowerBuyer 6.5 (Japan).zip` | Not importable | No Pocket PC program was found in it. |
| [PPC Planet CE Archive](https://archive.org/details/ppc-planet-ce) | `Dell Axim X5 Keyboard Driver.EXE` | Not importable | This is a desktop Windows program, and no Pocket PC game could be taken out of it. |
| [Prey 2006 Zeebo and Pocket PC mobile games](https://archive.org/details/prey-2006-mobile-Zeebo-PocketPC) | `Prey_Mobile.rar` | Not importable | No Pocket PC program was found in it. |
| [Psion Workabout Pro G1 firmware](https://archive.org/details/psion-workabout-pro-g1-firmware) | `Windows CE.net-K186m.zip` | Not importable | No Pocket PC program was found in it. |
| [PuzzleIT](https://archive.org/details/tucows_265865_PuzzleIT) | `puzzleitv100.exe` | Not importable | This is a desktop Windows program, and no Pocket PC game could be taken out of it. |
| [QuickAgenda](https://archive.org/details/tucows_216979_QuickAgenda) | `quickagenda_2002.zip` | Not importable | No Pocket PC program was found in it. |
| [QuickAgenda 2002](https://archive.org/details/tucows_243925_QuickAgenda_2002) | `quickagenda_2002.zip` | Not importable | No Pocket PC program was found in it. |
| [QuickCleanUp AddIn](https://archive.org/details/tucows_214806_QuickCleanUp_AddIn) | `QuickCleanUpAddIn.zip` | Not importable | No Pocket PC program was found in it. |
| [QuickTasks](https://archive.org/details/tucows_216978_QuickTasks) | `quicktasks.zip` | Not importable | No Pocket PC program was found in it. |
| [QuickTasks 2002](https://archive.org/details/tucows_243721_QuickTasks_2002) | `quicktasks_2002.zip` | Not importable | No Pocket PC program was found in it. |
| [Quote Ticker Bar](https://archive.org/details/tucows_32216_Quote_Ticker_Bar) | `qtbce.exe` | Not importable | This is a desktop Windows program, and no Pocket PC game could be taken out of it. |
| [Radium (PocketPC) Demo](https://archive.org/details/radiumpocketpcdemo) | `RadiumPocketPCDemo.exe` | Not importable | This is a desktop Windows program, and no Pocket PC game could be taken out of it. |
| [Rayman (Japan) (En,Ja)](https://archive.org/details/Nova_RaymanPPC_Japan) | `[LOGS] Rayman (Japan) (En,Ja).zip` | Not importable | No Pocket PC program was found in it. |
| [Rayman Ultimate (USA)](https://archive.org/details/Nova_RaymanUltimate-PPC_USA) | `[LOGS] Rayman Ultimate (USA).zip` | Not importable | No Pocket PC program was found in it. |
| [RC4, RC5 Multi-Rom V2](https://archive.org/details/multiromv2) | `RC5 Multi-Rom.rar` | Not importable | No Pocket PC program was found in it. |
| [RC4, RC5 Multi-Rom V2](https://archive.org/details/multiromv2) | `RC4 Multi-Rom v2.rar` | Not importable | No Pocket PC program was found in it. |
| [Red Phone](https://archive.org/details/tucows_173150_Red_Phone) | `RedPhone_Demo.zip` | Not importable | No Pocket PC program was found in it. |
| [RegEditCE](https://archive.org/details/tucows_32378_RegEditCE) | `RegEditCE.exe` | Not importable | This is a desktop Windows program, and no Pocket PC game could be taken out of it. |
| [Release Dates](https://archive.org/details/tucows_52026_Release_Dates) | `reldatedemo.zip` | Not importable | No Pocket PC program was found in it. |
| [ReleaseDates](https://archive.org/details/tucows_54695_ReleaseDates) | `reldatedemo.zip` | Not importable | No Pocket PC program was found in it. |
| [RemindMe](https://archive.org/details/tucows_169173_RemindMe) | `REMSetup.exe` | Not importable | This is a desktop Windows program, and no Pocket PC game could be taken out of it. |
| [Remote Control](https://archive.org/details/tucows_219697_Remote_Control) | `remotecontrol.exe` | Not importable | This is a desktop Windows program, and no Pocket PC game could be taken out of it. |
| [RemoteWAVPlayer](https://archive.org/details/tucows_226382_RemoteWAVPlayer) | `WAVPlayerSampleClient.zip` | Not importable | No Pocket PC program was found in it. |
| [Resco Keyboard](https://archive.org/details/tucows_149921_Resco_Keyboard) | `keyboardpro_p.exe` | Not importable | This is a desktop Windows program, and no Pocket PC game could be taken out of it. |
| [Resco Utility Package](https://archive.org/details/tucows_216199_Resco_Utility_Package) | `UtilPack_p.exe` | Not importable | This is a desktop Windows program, and no Pocket PC game could be taken out of it. |
| [Resco WordGame](https://archive.org/details/tucows_237205_Resco_WordGame) | `wordgame_p.exe` | Not importable | This is a desktop Windows program, and no Pocket PC game could be taken out of it. |
| [Resco Zipper](https://archive.org/details/tucows_167100_Resco_Zipper) | `zipper_p.exe` | Not importable | This is a desktop Windows program, and no Pocket PC game could be taken out of it. |
| [RipanGo](https://archive.org/details/tucows_329856_RipanGo) | `ripango_setup.exe` | Not importable | This is a desktop Windows program, and no Pocket PC game could be taken out of it. |
| [RocketElite (USA) (En,Fr,De)](https://archive.org/details/Nova_RocketElite_USA) | `RocketElite (USA) (En,Fr,De)/[LOGS] RocketElite (USA) (En,Fr,De).zip` | Not importable | No Pocket PC program was found in it. |
| [Rom-Man Technologies' Windows CE Professional (USA)](https://archive.org/details/Nova_RomManWindowsCE_USA) | `[LOGS] Rom-Man Technologies' Windows CE Professional (USA).zip` | Not importable | No Pocket PC program was found in it. |
| [ROMs and Utilities for the HTC Advantage X7500 and X7510](https://archive.org/details/roms-and-utilities-htc-advantage) | `mac1.zip` | Not importable | No Pocket PC program was found in it. |
| [ROMs and Utilities for the HTC Advantage X7500 and X7510](https://archive.org/details/roms-and-utilities-htc-advantage) | `battdrvr.zip` | Not importable | No Pocket PC program was found in it. |
| [Salsa Studio Mild Edition](https://archive.org/details/tucows_225436_Salsa_Studio_Mild_Edition) | `SalsaStudio26PPCMild.EXE` | Not importable | This is a desktop Windows program, and no Pocket PC game could be taken out of it. |
| [Scotty FTP](https://archive.org/details/tucows_32219_Scotty_FTP) | `scottyFTP_hpc.exe` | Not importable | This is a desktop Windows program, and no Pocket PC game could be taken out of it. |
| [ScreenPad](https://archive.org/details/tucows_59299_ScreenPad) | `ScreenPad.zip` | Not importable | No Pocket PC program was found in it. |
| [Series 5 vs Windows CE](https://archive.org/details/tucows_39071_Series_5_vs_Windows_CE) | `s5vsce.zip` | Not importable | No Pocket PC program was found in it. |
| [ShadowGate Classic (USA)](https://archive.org/details/Nova_ShadowGateClassic_USA) | `[LOGS] ShadowGate Classic (USA).zip` | Not importable | No Pocket PC program was found in it. |
| [Shazam for PC and Mobile 4.7.9.0 for Windows 10 version 1507 32-Bit and Mobile](https://archive.org/details/shazam-for-pc-and-mobile-4.7.9.0-for-windows-10-version-1507-32-bit-and-mobile) | `ShazamEntertainmentLtd.Shazam_4.7.9.0_neutral_~_pqbynwjfrbcg4.zip` | Not importable | No Pocket PC program was found in it. |
| [Shellkey](https://archive.org/details/tucows_32666_Shellkey) | `Shellkey.zip` | Not importable | No Pocket PC program was found in it. |
| [Sign-On](https://archive.org/details/tucows_208881_Sign-On) | `SignOnPPCTrial.exe` | Not importable | This is a desktop Windows program, and no Pocket PC game could be taken out of it. |
| [SimCity 2000 for Pocket PC](https://archive.org/details/sim-city-2000_202009) | `SimCity 2000.zip` | Not importable | No Pocket PC program was found in it. |
| [SimCity 2000 for Pocket PC (USA)](https://archive.org/details/Nova_SimCity2000PPC_USA) | `SimCity 2000 for Pocket PC (USA)/[LOGS] SimCity 2000 for Pocket PC (USA).zip` | Not importable | No Pocket PC program was found in it. |
| [Simcity 2000 Pocket PC](https://archive.org/details/simcity2k_pocketpc) | `ZIOSoft SimCity 2000/SimCity2000.exe` | Not importable | This is a desktop Windows program, and no Pocket PC game could be taken out of it. |
| [Simple SMS](https://archive.org/details/tucows_208476_Simple_SMS) | `SimpleSMSDemo.zip` | Not importable | No Pocket PC program was found in it. |
| [Slim Pocket FTP](https://archive.org/details/tucows_295627_Slim_Pocket_FTP) | `TrialPocketFTP.zip` | Not importable | No Pocket PC program was found in it. |
| [SolPack](https://archive.org/details/tucows_56259_SolPack) | `solpack_ppc.exe` | Not importable | This is a desktop Windows program, and no Pocket PC game could be taken out of it. |
| [SolPack 2](https://archive.org/details/tucows_59131_SolPack_2) | `solpack2_ppc.exe` | Not importable | This is a desktop Windows program, and no Pocket PC game could be taken out of it. |
| [Sony Ericsson Xperia X1 HardSPL V1](https://archive.org/details/x-1-hard-spl-v-1) | `ruusigned_072stockspl.zip` | Not importable | No Pocket PC program was found in it. |
| [Sony Ericsson Xperia X1 HardSPL V1](https://archive.org/details/x-1-hard-spl-v-1) | `xperia078_2.zip` | Not importable | No Pocket PC program was found in it. |
| [SPB Backup 1.6.2 for Pocket PC](https://archive.org/details/spb-backup-ppc) | `SpbBackup_setup.exe` | Not importable | This is a desktop Windows program, and no Pocket PC game could be taken out of it. |
| [SpeedClean](https://archive.org/details/tucows_216460_SpeedClean) | `speedclean.zip` | Not importable | No Pocket PC program was found in it. |
| [sportinglife eCalendar Companion](https://archive.org/details/tucows_54488_sportinglife_eCalendar_Companion) | `setup.exe` | Not importable | This is a desktop Windows program, and no Pocket PC game could be taken out of it. |
| [sportinglife eCalendar Companion](https://archive.org/details/tucows_54490_sportinglife_eCalendar_Companion) | `sportinglife.zip` | Not importable | No Pocket PC program was found in it. |
| [sportinglife eCalendar Companion](https://archive.org/details/tucows_54646_sportinglife_eCalendar_Companion) | `sportlife_ecal.zip` | Not importable | No Pocket PC program was found in it. |
| [SpreadCE for Windows CE 3.12 2022-08-03](https://archive.org/details/spreadce-312-20220803) | `PPC/SCEFile.zip` | Not importable | No Pocket PC program was found in it. |
| [Standard Time](https://archive.org/details/tucows_169876_Standard_Time) | `standardtimece100.zip` | Not importable | No Pocket PC program was found in it. |
| [STL Adaptation](https://archive.org/details/tucows_32380_STL_Adaptation) | `stl_eVC.zip` | Not importable | No Pocket PC program was found in it. |
| [Sunnysoft Backup Manager](https://archive.org/details/tucows_249076_Sunnysoft_Backup_Manager) | `Sunnysoft_BackupManager_US.zip` | Not importable | No Pocket PC program was found in it. |
| [SyncTalk FX](https://archive.org/details/tucows_219048_SyncTalk_FX) | `synctalk.zip` | Not importable | No Pocket PC program was found in it. |
| [SYWARE Six Pack](https://archive.org/details/tucows_196606_SYWARE_Six_Pack) | `sixpack.zip` | Not importable | No Pocket PC program was found in it. |
| [TaskPro Navigator](https://archive.org/details/tucows_222192_TaskPro_Navigator) | `devonetaskpronavtrial.exe` | Not importable | This is a desktop Windows program, and no Pocket PC game could be taken out of it. |
| [TauBetaPi Advanced Explorer](https://archive.org/details/tucows_308022_TauBetaPi_Advanced_Explorer) | `advexpr_setup.exe` | Not importable | This is a desktop Windows program, and no Pocket PC game could be taken out of it. |
| [TeleType GPS](https://archive.org/details/tucows_32685_TeleType_GPS) | `ttsgpsdemo.zip` | Not importable | No Pocket PC program was found in it. |
| [Tetris Classic Game Pak for Pocket PCs](https://archive.org/details/tetris-classic-game-pak) | `setup.exe` | Not importable | This is a desktop Windows program, and no Pocket PC game could be taken out of it. |
| [Time and Chaos](https://archive.org/details/tucows_260904_Time_and_Chaos) | `chaossetup.exe` | Not importable | This is a desktop Windows program, and no Pocket PC game could be taken out of it. |
| [TimeTTracker 2000](https://archive.org/details/tucows_231046_TimeTTracker_2000) | `timettracker.exe` | Not importable | This is a desktop Windows program, and no Pocket PC game could be taken out of it. |
| [Tipy a triky pro Pocket PC CD-ROM](https://archive.org/details/tipytrikyppccd) | `Tipy-a-Triky-pro-PocketPC-covermount_dic-files.zip` | Not importable | No Pocket PC program was found in it. |
| [TOEIC Test no Tame no Pocket PC Soft (Japan) (En,Ja)](https://archive.org/details/Nova_TOEICTest-PPC_JPN) | `[LOGS] TOEIC Test no Tame no Pocket PC Soft (Japan) (En,Ja).zip` | Not importable | No Pocket PC program was found in it. |
| [Tomb Raider for Pocket PC (USA)](https://archive.org/details/Nova_TombRaiderPPC_USA) | `Tomb Raider for Pocket PC (USA)/[LOGS] Tomb Raider for Pocket PC (USA).zip` | Not importable | No Pocket PC program was found in it. |
| [Tony Hawk's Pro Skater 2 (USA)](https://archive.org/details/Nova_THPS2PPC_USA) | `Tony Hawk's Pro Skater 2 (USA)/[LOGS] Tony Hawk's Pro Skater 2 (USA).zip` | Not importable | No Pocket PC program was found in it. |
| [Toshiba e400 Pocket PC 2003 SE Firmware Upgrade](https://archive.org/details/e400_ppc03se_firmware) | `ppc_usb10.exe` | Not importable | This is a desktop Windows program, and no Pocket PC game could be taken out of it. |
| [Toshiba e400 Pocket PC 2003 SE Firmware Upgrade](https://archive.org/details/e400_ppc03se_firmware) | `e400_ppc03se.exe` | Not importable | This is a desktop Windows program, and no Pocket PC game could be taken out of it. |
| [Toshiba GENIO e830, e830w ROM Update(V1.0411.0411 Japanese)](https://archive.org/details/e830r0411) | `e830r0411.exe` | Not importable | This is a desktop Windows program, and no Pocket PC game could be taken out of it. |
| [Tracer Pocket PC](https://archive.org/details/tucows_362917_Tracer_Pocket_PC) | `tracer_ppc.exe` | Not importable | This is a desktop Windows program, and no Pocket PC game could be taken out of it. |
| [Tricorder](https://archive.org/details/tucows_216984_Tricorder) | `tricorder.zip` | Not importable | No Pocket PC program was found in it. |
| [Ultima Underworld - The Stygian Abyss (USA)](https://archive.org/details/Nova_UltimaUnderworldStygianPPC_USA) | `Ultima Underworld - The Stygian Abyss (USA)/[LOGS] Ultima Underworld - The Stygian Abyss (USA).zip` | Not importable | No Pocket PC program was found in it. |
| [Vegas Casino - Vol. 1 & 2 (Canada)](https://archive.org/details/Nova_Vegas-Casino-Vol-1-2-PPC_CAN) | `[LOGS] Vegas Casino - Vol. 1 & 2 (Canada).zip` | Not importable | No Pocket PC program was found in it. |
| [Veo Photo Traveler For Pocket PC Driver Installation](https://archive.org/details/veo-photo-traveler-for-pocket-pc-driver) | `Veo Photo Traveler for Pocket PC Setup/setup.exe` | Not importable | This is a desktop Windows program, and no Pocket PC game could be taken out of it. |
| [VideoList Plus](https://archive.org/details/tucows_336125_VideoList_Plus) | `videolistplus-setup.exe` | Not importable | This is a desktop Windows program, and no Pocket PC game could be taken out of it. |
| [Viewsonic V37 PDA Companion CD](https://archive.org/details/wceusbsh_202610) | `SETUP.EXE` | Not importable | This is a desktop Windows program, and no Pocket PC game could be taken out of it. |
| [Virtual iPAQ](https://archive.org/details/tucows_151843_Virtual_iPAQ) | `v_ipaq_free.zip` | Not importable | No Pocket PC program was found in it. |
| [VITO Remote](https://archive.org/details/tucows_295742_VITO_Remote) | `VitoRemote.zip` | Not importable | No Pocket PC program was found in it. |
| [VITO SketchArtist](https://archive.org/details/tucows_296842_VITO_SketchArtist) | `VitoSketchArtist.zip` | Not importable | No Pocket PC program was found in it. |
| [VITO SmartMap](https://archive.org/details/tucows_276195_VITO_SmartMap) | `VitoSmartMap_Std.zip` | Not importable | No Pocket PC program was found in it. |
| [VITO SoundExplorer](https://archive.org/details/tucows_295740_VITO_SoundExplorer) | `VitoSoundExplorer.zip` | Not importable | No Pocket PC program was found in it. |
| [VoiCErun](https://archive.org/details/tucows_196372_VoiCErun) | `VoiCErun_254_demo_arm.exe` | Not importable | This is a desktop Windows program, and no Pocket PC game could be taken out of it. |
| [vorbislq for TCPMP v0.71 (v0.71) (Windows Mobile Port)](https://archive.org/details/vorbislq.ARM_SP) | `vorbislq.ARM_SP.CAB` | Not importable | No Pocket PC program was found in it. |
| [Waba](https://archive.org/details/tucows_33600_Waba) | `wabavm-palm.10b.zip` | Not importable | No Pocket PC program was found in it. |
| [Walls and Balls](https://archive.org/details/tucows_332838_Walls_and_Balls) | `WnB.zip` | Not importable | No Pocket PC program was found in it. |
| [Wave Drummer](https://archive.org/details/tucows_59133_Wave_Drummer) | `WaveDrummer.zip` | Not importable | No Pocket PC program was found in it. |
| [Windows Media Play Pack for Pocket PC (USA)](https://archive.org/details/Nova_WindowsPlayPackPPC_USA) | `Windows Media Play Pack for Pocket PC (USA)/[LOGS] Windows Media Play Pack for Pocket PC (USA).zip` | Not importable | No Pocket PC program was found in it. |
| [Windows Media Player for Mobile and Pocket PC](https://archive.org/details/WMPlayer-Mobile) | `WMPlayer-Mobile.rar` | Not importable | No Pocket PC program was found in it. |
| [Windows Mobile 2002 and 2003 (SE) x86 VMDK images](https://archive.org/details/winmobile-2003-se-v-3) | `pocketpc2002 V2.zip` | Not importable | No Pocket PC program was found in it. |
| [Windows Mobile 2002 and 2003 (SE) x86 VMDK images](https://archive.org/details/winmobile-2003-se-v-3) | `smartphone2002 V2.zip` | Not importable | No Pocket PC program was found in it. |
| [Windows Mobile 5.0 (2024 Update) (KB944326)](https://archive.org/details/new-caracas-tzwm-5.-sp) | `NewCaracasTZWM5.PPC.CAB` | Not importable | No Pocket PC program was found in it. |
| [Windows Mobile 5.0 (2024 Update) (KB944326)](https://archive.org/details/new-caracas-tzwm-5.-sp) | `NewCaracasTZWM5.SP.CAB` | Not importable | No Pocket PC program was found in it. |
| [Windows Mobile 6.1 for HTC Typhoon, Feeler, Amadeus](https://archive.org/details/wm-6.1-v.-1.6-amadeus) | `mtty1.42.exe` | Not importable | This is a desktop Windows program, and no Pocket PC game could be taken out of it. |
| [Windows Mobile 6.x build collection](https://archive.org/details/wm6.x-build-collection) | `21681/xip_21681.rar` | Not importable | No Pocket PC program was found in it. |
| [Windows Mobile 6.x build collection](https://archive.org/details/wm6.x-build-collection) | `23148/XIP_23148.rar` | Not importable | No Pocket PC program was found in it. |
| [Windows Mobile Device Emulator](https://archive.org/details/wmemu) | `Emulator/devemu.exe` | Not importable | This is a desktop Windows program, and no Pocket PC game could be taken out of it. |
| [Windows Phone 7/8 Games Collection: PDALife Archive (A,B,C,D)](https://archive.org/details/windows-phone-pdalife) | `Avalonwar.rar` | Not importable | No Pocket PC program was found in it. |
| [Windows Phone Desktop app for Mac](https://archive.org/details/windows-phone-mac) | `Windows Phone.zip` | Not importable | No Pocket PC program was found in it. |
| [Windows Phone FFU Archive](https://archive.org/details/lumiadb) | `RM-943/RM-943.zip` | Not importable | No Pocket PC program was found in it. |
| [Windows Phone FFU Archive](https://archive.org/details/lumiadb) | `RM-826/RM-826.zip` | Not importable | No Pocket PC program was found in it. |
| [Windows Phone FFU Archive 2](https://archive.org/details/lumiadb2) | `RM-915/RM-915.zip` | Not importable | No Pocket PC program was found in it. |
| [Windows Phone FFU Archive 2](https://archive.org/details/lumiadb2) | `RM-914/RM-914.zip` | Not importable | No Pocket PC program was found in it. |
| [Windows Phone FFU Archive 3](https://archive.org/details/lumiadb3) | `RM-994/RM-994.zip` | Not importable | No Pocket PC program was found in it. |
| [Windows Phone FFU Archive 3](https://archive.org/details/lumiadb3) | `RM-885/RM-885.zip` | Not importable | No Pocket PC program was found in it. |
| [Windows Phone FFU Archive 4](https://archive.org/details/lumiadb4) | `RM-893/RM-893.zip` | Not importable | No Pocket PC program was found in it. |
| [Windows Phone FFU Archive 4](https://archive.org/details/lumiadb4) | `RM-820/RM-820.zip` | Not importable | No Pocket PC program was found in it. |
| [Windows Phone FFU Archive 5](https://archive.org/details/lumiadb5) | `RM-846/RM-846.zip` | Not importable | No Pocket PC program was found in it. |
| [Windows Phone FFU Archive 5](https://archive.org/details/lumiadb5) | `RM-1091/RM-1091.zip` | Not importable | No Pocket PC program was found in it. |
| [Windows Phone SDK 7.8 and 8.0 (Russian)](https://archive.org/details/WindowsPhoneSDK-7.8) | `Windows Phone SDK 7.8 Update/Packages/Images_7.8/rus/cab1.cab` | Not importable | No Pocket PC program was found in it. |
| [Windows SideShow Gadgets, Emulator & Windows Mobile Application](https://archive.org/details/windows-sideshow-gadgets-emulator-windows-mobile-application) | `Windows SideShow Gadgets/RSS Feed Viewer and Stocks.zip` | Not importable | No Pocket PC program was found in it. |
| [Windows SideShow Gadgets, Emulator & Windows Mobile Application](https://archive.org/details/windows-sideshow-gadgets-emulator-windows-mobile-application) | `Windows SideShow Emulators & Mobile Application/WindowsSideShowforWindowsMobile.exe` | Not importable | This is a desktop Windows program, and no Pocket PC game could be taken out of it. |
| [WinDriver for Windows](https://archive.org/details/tucows_359133_WinDriver_for_Windows) | `WD622.EXE` | Not importable | This is a desktop Windows program, and no Pocket PC game could be taken out of it. |
| [WinDriver for Windows CE](https://archive.org/details/tucows_32388_WinDriver_for_Windows_CE) | `WD622CE.EXE` | Not importable | This is a desktop Windows program, and no Pocket PC game could be taken out of it. |
| [WingX (USA)](https://archive.org/details/Nova_WingX_USA) | `WingX (USA)/[LOGS] WingX (USA).zip` | Not importable | No Pocket PC program was found in it. |
| [Word Search](https://archive.org/details/tucows_55861_Word_Search) | `wsearch_ppc.exe` | Not importable | This is a desktop Windows program, and no Pocket PC game could be taken out of it. |
| [WordAssociation](https://archive.org/details/tucows_68095_WordAssociation) | `WordAssociation.zip` | Not importable | No Pocket PC program was found in it. |
| [WPBackup v1.0](https://archive.org/details/wpbackup) | `WPBackup_v1.0.rar` | Not importable | No Pocket PC program was found in it. |
| [WyndMail](https://archive.org/details/tucows_32239_WyndMail) | `wmwce11.zip` | Not importable | No Pocket PC program was found in it. |
| [X-Lite - Soft SIP VOIP Phone](https://archive.org/details/tucows_309984_X-Lite_-_Soft_SIP_VOIP_Phone) | `X-Lite_Install.exe` | Not importable | This is a desktop Windows program, and no Pocket PC game could be taken out of it. |
| [XTNDConnect PC](https://archive.org/details/tucows_32409_XTNDConnect_PC) | `xtndconn.exe` | Not importable | This is a desktop Windows program, and no Pocket PC game could be taken out of it. |
| [Yacht](https://archive.org/details/tucows_71004_Yacht) | `yacht.zip` | Not importable | The archive is damaged or cannot be read. |
| [ZIO FunPack (USA)](https://archive.org/details/Nova_ZIOFunPackPPC_USA) | `[LOGS] ZIO FunPack (USA).zip` | Not importable | No Pocket PC program was found in it. |
| [ZIOGolf 2 (USA)](https://archive.org/details/Nova_ZIOGolf2_USA) | `ZIOGolf 2 (USA)/[LOGS] ZIOGolf 2 (USA).zip` | Not importable | No Pocket PC program was found in it. |
| [ZTERM](https://archive.org/details/tucows_32235_ZTERM) | `ztce.exe` | Not importable | This is a desktop Windows program, and no Pocket PC game could be taken out of it. |
| [Энциклопедия Pocket PC [Disc,Scans]](https://archive.org/details/enciclopedia-pocket-pc-iddk) | `Энциклопедия Pocket PC [ИДДК].rar` | Not importable | No Pocket PC program was found in it. |
| [小米手机4刷机Windows10工具包Mi Flash 20150601](https://archive.org/details/mi-flash-20150601) | `MiFlash20150601.zip` | Not importable | No Pocket PC program was found in it. |

## ⚪ Not tested (420)

The archive.org item has no package, or the download failed.

| Title | File | Result | Notes |
|---|---|---|---|
| [1000 Programmi Per Il Tuo Palmare N 9](https://archive.org/details/1000-programmi-per-il-tuo-palmare-n-9) | `` | No package | No zip, cab, exe or rar in the item |
| [1000 Programmi Per Il Tuo Palmare N. 10](https://archive.org/details/1000-programmi-per-il-tuo-palmare) | `` | No package | No zip, cab, exe or rar in the item |
| [114 네비(포켓나비)](https://archive.org/details/114_poketnavi) | `` | No package | No zip, cab, exe or rar in the item |
| [1333 Applications palmOS](https://archive.org/details/1333-applications) | `` | No package | No zip, cab, exe or rar in the item |
| [200+ Games for PDA](https://archive.org/details/200-plus-games-for-pda) | `` | No package | No zip, cab, exe or rar in the item |
| [224 Programmi Per PalmOS e Pocket PC Notebook & PDA](https://archive.org/details/224-programmi-per-il-palmmare) | `` | No package | No zip, cab, exe or rar in the item |
| [2in1CC](https://archive.org/details/tucows_257663_2in1CC) | `setup-arm-trial.exe` | Download failed | Download from archive.org failed |
| [46 Working Full Progs For Hp Jornada 720 Handheld Windows CE Torrent](https://archive.org/details/46WorkingFullProgsForHpJornada720HandheldWindowsCETorrent) | `` | No package | No zip, cab, exe or rar in the item |
| [750 Programmi Per Il Tuo Palmare](https://archive.org/details/750-programmi-per-il-tuo-palmare) | `` | No package | No zip, cab, exe or rar in the item |
| [8.10.14114.125.WPB_CXE_R1.20140604-1634](https://archive.org/details/8.10.14114.125.WPB_CXE_R1.20140604-1634) | `` | No package | No zip, cab, exe or rar in the item |
| [800 Programmi Per Il Tuo Palmare N 11](https://archive.org/details/800-programmi-per-il-tuo-palmare-n-11) | `` | No package | No zip, cab, exe or rar in the item |
| [900 Programmi Per Il Tuo Palmare N 12](https://archive.org/details/900-programmi-per-il-tuo-palmare-n-12) | `` | No package | No zip, cab, exe or rar in the item |
| [[Noaen] Redump - Pocket PC](https://archive.org/details/noaen-redump-pocket-pc) | `` | No package | No zip, cab, exe or rar in the item |
| [A backup of Forestar's Palmtop/PocketPC Page](https://archive.org/details/forestar) | `` | No package | No zip, cab, exe or rar in the item |
| [Acer n35 Handheld Companion CD](https://archive.org/details/acer-n35-cd) | `` | No package | No zip, cab, exe or rar in the item |
| [ActiveSync 3.1](https://archive.org/details/tucows_32548_ActiveSync_3.1) | `msasync31.exe` | Download failed | Download from archive.org failed |
| [ActiveSync 4.5](https://archive.org/details/activesync45) | `` | No package | No zip, cab, exe or rar in the item |
| [ActiveSync 4.5 Italian](https://archive.org/details/activesync45ita) | `` | No package | No zip, cab, exe or rar in the item |
| [AND PathFinder AND FlightFinder para Pocket PC (Cassiopeia)](https://archive.org/details/AND_PathFinder_AND_FlightFinder_para_Pocket_PC_Cassiopeia) | `` | No package | No zip, cab, exe or rar in the item |
| [AND PathFinder AND Flightfinder Para Pocket PC (Wings)](https://archive.org/details/AND_PathFinder_AND_Flightfinder_Para_Pocket_PC_Wings) | `` | No package | No zip, cab, exe or rar in the item |
| [Android 2.2 for HTC MAX 4G](https://archive.org/details/xdandroid-2.2.-aosp.-frx-06-4quartz-0.4.1.7z) | `` | No package | No zip, cab, exe or rar in the item |
| [Android Fathom](https://archive.org/details/androidfathomcyanogenmod12) | `` | No package | No zip, cab, exe or rar in the item |
| [ARCO EXPERT CONNECT IUN 2005](https://archive.org/details/arcoexpert-connect-iun-2005) | `` | No package | No zip, cab, exe or rar in the item |
| [ARCSOFT Photo Base For Pocket PC](https://archive.org/details/arcsoft_202605) | `` | No package | No zip, cab, exe or rar in the item |
| [Arvale Short Tales + Emulator](https://archive.org/details/arvale-short-tales-emulator) | `` | No package | No zip, cab, exe or rar in the item |
| [Asphalt 4: Elite Racing - Omnia Port (Android/Windows)](https://archive.org/details/a4_port_aw) | `Asphalt4_Win.zip` | Download failed | Download from archive.org failed |
| [Asus MyPal A639/A626/A686/A696 Getting Started CD v1.0 + A626/A686/A696 Bonus CD v1.0 + Skype for Pocket PC installation CD-ROM](https://archive.org/details/asus_mypal) | `` | No package | No zip, cab, exe or rar in the item |
| [AT&T Black Jack™ II Getting Started CD (GH46-00553A)](https://archive.org/details/att-black-jack-ii-gh46-00553A) | `` | No package | No zip, cab, exe or rar in the item |
| [Audiovox RTM8000 CD-ROM](https://archive.org/details/audio-vox-cd-8000) | `` | No package | No zip, cab, exe or rar in the item |
| [Audiovox SMT5600 CD (HTC Typhoon)](https://archive.org/details/audiovox-smt-5600-cd-extracted) | `` | No package | No zip, cab, exe or rar in the item |
| [Baldhead Gaming](https://archive.org/details/tucows_251304_Baldhead_Gaming) | `` | No package | No zip, cab, exe or rar in the item |
| [Berlitz Mandarin Chinese & Japanese Premier](https://archive.org/details/berlitz-chinese-japanese) | `` | No package | No zip, cab, exe or rar in the item |
| [Berlitz PDA Italian Premier](https://archive.org/details/berlitzpdaitalianpremier) | `` | No package | No zip, cab, exe or rar in the item |
| [Berlitz PDA Mandrin Chinese And Japnese Premier CD](https://archive.org/details/berlitzpdamandrinchineseandjapnesepremier) | `` | No package | No zip, cab, exe or rar in the item |
| [Best Of Everything Pocket PC Magazine ( Pocket PCMag)( 2003)](https://archive.org/details/best-of-everything-pocket-pc-magazine-pocket-pcmag-2003-iso) | `` | No package | No zip, cab, exe or rar in the item |
| [Best of Everything Pocket PC Magazine (PocketPCMag)(2003)](https://archive.org/details/Best_of_Everything_Pocket_PC_Magazine_PocketPCMag_2003) | `` | No package | No zip, cab, exe or rar in the item |
| [BigClock](https://archive.org/details/tucows_233439_BigClock) | `bigclock.zip` | Download failed | Download from archive.org failed |
| [BigClock](https://archive.org/details/tucows_32530_BigClock) | `BigClockPPCMIPS.zip` | Download failed | Download from archive.org failed |
| [BijiEditor](https://archive.org/details/tucows_32566_BijiEditor) | `bijihpcm.zip` | Download failed | Download from archive.org failed |
| [BookWorm for Palm OS and Pocket PCs](https://archive.org/details/bookworm-ppc) | `` | No package | No zip, cab, exe or rar in the item |
| [Brain Gym](https://archive.org/details/tucows_237678_Brain_Gym) | `braingym.exe` | Download failed | Download from archive.org failed |
| [Bullant Remote](https://archive.org/details/tucows_286106_Bullant_Remote) | `BullantRemoteForPocketPCARM.exe` | Download failed | Download from archive.org failed |
| [C4PDA Scrape - Windows Mobile](https://archive.org/details/c4pda-scrape-wm) | `` | No package | No zip, cab, exe or rar in the item |
| [Calc98](https://archive.org/details/tucows_212356_Calc98) | `` | No package | No zip, cab, exe or rar in the item |
| [Call Of Duty 2 Windows Mobile / Pocket PC. 7z](https://archive.org/details/call-of-duty-2-ppc.-7z) | `` | No package | No zip, cab, exe or rar in the item |
| [Casio Cassiopeia EM-500 Companion Discs](https://archive.org/details/casio-cassiopeia-em-500-companion-discs) | `` | No package | No zip, cab, exe or rar in the item |
| [Casio Cassiopeia EM-500 ROM Dump](https://archive.org/details/casio-cassiopeia-em-500-rom-dump) | `` | No package | No zip, cab, exe or rar in the item |
| [Casio EM-500 ROM dump](https://archive.org/details/casio-em500-rom-dump) | `` | No package | No zip, cab, exe or rar in the item |
| [Cassiopeia Software Applications 3.0R.3](https://archive.org/details/cassiopeia-software-applications-3.0-r.-3) | `` | No package | No zip, cab, exe or rar in the item |
| [CD Passos Iniciais para HP iPAQ Pocket PC linha hx2000 Versão 2.0(Brazil)](https://archive.org/details/office-10_20240128) | `` | No package | No zip, cab, exe or rar in the item |
| [CE3270](https://archive.org/details/tucows_232365_CE3270) | `` | No package | No zip, cab, exe or rar in the item |
| [CECollections v2.02 [NT351] [Windows CE Emulators]](https://archive.org/details/cecollections-2.02-v-202-r-001.iso.-7z) | `` | No package | No zip, cab, exe or rar in the item |
| [CETuner](https://archive.org/details/tucows_32478_CETuner) | `CETunerE.exe` | Download failed | Download from archive.org failed |
| [Change Appointment Timezone](https://archive.org/details/tucows_150321_Change_Appointment_Timezone) | `ChgTzoneApptARM.zip` | Download failed | Download from archive.org failed |
| [CheckLister XP](https://archive.org/details/tucows_298139_CheckLister_XP) | `CheckXP.exe` | Download failed | Download from archive.org failed |
| [COD 2: Windows Mobile Edition (READ DESC)](https://archive.org/details/cod-2-windows-mobile) | `` | No package | No zip, cab, exe or rar in the item |
| [ColorMe](https://archive.org/details/tucows_32480_ColorMe) | `ColorMe_mips.zip` | Download failed | Download from archive.org failed |
| [Companion CD for Samsung Omnia II](https://archive.org/details/samsung_omnia_i920_companion) | `` | No package | No zip, cab, exe or rar in the item |
| [Compaq Aero 2100 Software CD](https://archive.org/details/compaq-aero-2100-software-cd) | `` | No package | No zip, cab, exe or rar in the item |
| [compaq c2010 driver set CD](https://archive.org/details/compaqc2010driversetc) | `` | No package | No zip, cab, exe or rar in the item |
| [Compaq I Paq H 3700 Driver CD September 2001](https://archive.org/details/compaq-i-paq-h-3700-driver-cd-september-2001) | `` | No package | No zip, cab, exe or rar in the item |
| [Compaq iPAQ H3600 Pocket PC software and reference guide](https://archive.org/details/h-3600-15v-4) | `` | No package | No zip, cab, exe or rar in the item |
| [Compaq iPAQ H3600 Upgrade Pocket PC 2002](https://archive.org/details/compaq-i-paq-h-3600-upgrade-pocket-pc-2002) | `` | No package | No zip, cab, exe or rar in the item |
| [Compaq iPAQ H3800 ROM](https://archive.org/details/nk-glamis-3812021-wwe) | `` | No package | No zip, cab, exe or rar in the item |
| [Compaq iPaq H3800 Series Companion CD](https://archive.org/details/compaq-ipaq-h3800-series-companion-cd) | `` | No package | No zip, cab, exe or rar in the item |
| [Compaq Ipaq pocket pc 2003 with outlook 2002](https://archive.org/details/compaq-ipaq) | `` | No package | No zip, cab, exe or rar in the item |
| [Compaq iPAQ Pocket PC Begleit-CD H3800 Familie](https://archive.org/details/20230603-205638) | `` | No package | No zip, cab, exe or rar in the item |
| [Compaq iPAQ Pocket PC Companion CD H3700 Series](https://archive.org/details/office-10_202307) | `` | No package | No zip, cab, exe or rar in the item |
| [Compaq iPAQ Pocket PC H3100 Series - Microsoft Pocket PC 2002 Upgrade](https://archive.org/details/pocket-pc-2002-h3100) | `` | No package | No zip, cab, exe or rar in the item |
| [Compaq iPAQ Pocket PC H3850 drivers and updates](https://archive.org/details/H3850) | `` | No package | No zip, cab, exe or rar in the item |
| [Conduits Pocket Player](https://archive.org/details/pocket-player-ppc) | `` | No package | No zip, cab, exe or rar in the item |
| [CoPilot Live 4 PPC CDs](https://archive.org/details/cplivedata) | `` | No package | No zip, cab, exe or rar in the item |
| [COSMI BIBLE To Go](https://archive.org/details/pdabible-091801) | `` | No package | No zip, cab, exe or rar in the item |
| [Crossbow HD2 V1.0 Final (WWE) Windows Mobile 6.1 ROM](https://archive.org/details/crossbow-hd-2-v1.0-final-wwe) | `` | No package | No zip, cab, exe or rar in the item |
| [Cuckoo's Call](https://archive.org/details/tucows_32486_Cuckoo_s_Call) | `cuckoo1210.exe` | Download failed | Download from archive.org failed |
| [Custom Windows Mobile 6.5.5 HD2 ROM's](https://archive.org/details/windowsmobile6.5.5HD2ROM-c_shekhar) | `` | No package | No zip, cab, exe or rar in the item |
| [CW Pocket Directory](https://archive.org/details/tucows_285220_CW_Pocket_Directory) | `` | No package | No zip, cab, exe or rar in the item |
| [Dark Wolf Windows Mobile 6.5 ROM for LG Incite CT810](https://archive.org/details/dark-wolf-28205-final) | `` | No package | No zip, cab, exe or rar in the item |
| [DataBeam](https://archive.org/details/tucows_32487_DataBeam) | `` | No package | No zip, cab, exe or rar in the item |
| [Dell Axim Executive Bluetooth KB Driver (2005)](https://archive.org/details/dell-axim-wireless-kb) | `` | No package | No zip, cab, exe or rar in the item |
| [Dell Axim Executive Bluetooth KB Driver (2006)](https://archive.org/details/dell-axim-executive-bluetooth-kb-driver-2006) | `` | No package | No zip, cab, exe or rar in the item |
| [Dell Axim Pocket PC 2003 Upgrade](https://archive.org/details/DellAximPocketPC2003Upgrade) | `` | No package | No zip, cab, exe or rar in the item |
| [Dell Axim X3 Pocket PC 2003 Companion CD](https://archive.org/details/office-10_202402) | `` | No package | No zip, cab, exe or rar in the item |
| [Dell Axim X3 Pocket PC Foldable Keyboard Driver](https://archive.org/details/aximkeyboard) | `` | No package | No zip, cab, exe or rar in the item |
| [Dell Axim X30 Windows Mobile 2003 Second Edition Companion CD](https://archive.org/details/dell-axim-x-30-windows-mobile-2003-second-edition-companion-cd) | `` | No package | No zip, cab, exe or rar in the item |
| [Dell Axim X5 Pocket PC 2003 Companion CD](https://archive.org/details/dell-axim-x5-pocket-pc-2003-companion-cd) | `` | No package | No zip, cab, exe or rar in the item |
| [Dell Axim X50 & X50v OEM Software CDs](https://archive.org/details/dell-axim-x50-x50v-oem-cd) | `` | No package | No zip, cab, exe or rar in the item |
| [Dell Axim X50(v) Windows Mobile 5 Upgrade](https://archive.org/details/dell-axim-x-50v-windows-mobile-5-upgrade) | `` | No package | No zip, cab, exe or rar in the item |
| [Dell Axim X51v X51 Pocket PC Getting Started CD](https://archive.org/details/axim-x-51-getting-started) | `` | No package | No zip, cab, exe or rar in the item |
| [Delorme XMap Street Atlas USA Handheld Edition v1.XH 2100 (2002)](https://archive.org/details/delorme-xmap-sa_usa_he-v1.xh-2100) | `` | No package | No zip, cab, exe or rar in the item |
| [DoobloDates](https://archive.org/details/tucows_232806_DoobloDates) | `dooblodv200.exe` | Download failed | Download from archive.org failed |
| [DOSBox V 0.73 XScale](https://archive.org/details/dosbox-v-0.73-xscale) | `` | No package | No zip, cab, exe or rar in the item |
| [Dragonstorm Windows Mobile 6.5 ROMs for LG IQ and LG eXpo](https://archive.org/details/dragonstorm-23134-iq) | `` | No package | No zip, cab, exe or rar in the item |
| [easySMS](https://archive.org/details/tucows_239742_easySMS) | `easySMS_allVer.zip` | Download failed | Download from archive.org failed |
| [eMbedded Visual C++ 4.0 JA + SP4](https://archive.org/details/ja-evc4) | `` | No package | No zip, cab, exe or rar in the item |
| [Energy.RAPHAEL.29022.Titanium.Jul.09 Pocket PC ROM](https://archive.org/details/energyrom-raphael-29022-titanium-jul-09) | `` | No package | No zip, cab, exe or rar in the item |
| [Esmertec JBed for Windows Mobile](https://archive.org/details/esmertec-jbed-for-windows-mobile) | `` | No package | No zip, cab, exe or rar in the item |
| [EverQuest for Pocket PC (Pocket EQ)](https://archive.org/details/pocket-eq) | `` | No package | No zip, cab, exe or rar in the item |
| [Falk Navigator Premium Edition For Pocket PC](https://archive.org/details/falk-navigator-premium-edition-for-pocket-pc) | `` | No package | No zip, cab, exe or rar in the item |
| [Falk Navigator Premium Edition Maps CD](https://archive.org/details/falk-navigator-premium-edition-maps-cd) | `` | No package | No zip, cab, exe or rar in the item |
| [Franklin Planner: Handheld Suite](https://archive.org/details/franklinhandheldsuite) | `` | No package | No zip, cab, exe or rar in the item |
| [Game Editor](https://archive.org/details/tucows_315172_Game_Editor) | `` | No package | No zip, cab, exe or rar in the item |
| [Games and Apps For Windows Mobile](https://archive.org/details/games-and-aps-for-windows-mobile) | `` | No package | No zip, cab, exe or rar in the item |
| [Games that work on Cassio/Cassiopeia E-125 PocketPC](https://archive.org/details/casio-e-125-games) | `` | No package | No zip, cab, exe or rar in the item |
| [Genesis Plus/Pocket PC. v0.3 Beta](https://archive.org/details/genpp03) | `` | No package | No zip, cab, exe or rar in the item |
| [GeometryManager](https://archive.org/details/tucows_32359_GeometryManager) | `GeometryManager.zip` | Download failed | Download from archive.org failed |
| [Getting Started CD for the HP iPAQ hx2000 Pocket PC Series 1st Edition (USA)](https://archive.org/details/HPiPAQhx2000_USA_1st_ed) | `` | No package | No zip, cab, exe or rar in the item |
| [Getting Started CD for the HP iPAQ™ r×5700 and r×5900 Travel Companion](https://archive.org/details/ipaq-rx5700-5900-cd) | `` | No package | No zip, cab, exe or rar in the item |
| [Getting Started Disc For GT-B7320 With Windows Mobile Software [Multilanguage]](https://archive.org/details/getting-started-disc-for-gt-b7320-with-windows-mobile-software-multilanguage) | `` | No package | No zip, cab, exe or rar in the item |
| [Google 2.0.295.80 (Windows Phone)](https://archive.org/details/google-2.0.295.80-wp) | `` | No package | No zip, cab, exe or rar in the item |
| [HanDBase Plus](https://archive.org/details/tucows_227273_HanDBase_Plus) | `handbaseplusppcdemo.exe` | Download failed | Download from archive.org failed |
| [Handmark Games Collection for Windows Mobile and PalmOS](https://archive.org/details/HandmarkGames) | `` | No package | No zip, cab, exe or rar in the item |
| [HandyZIP](https://archive.org/details/tucows_153373_HandyZIP) | `hz185ppe.exe` | Download failed | Download from archive.org failed |
| [Hawking Technolofy CompactFlash Modem H-CF690V Driver (v3.05)](https://archive.org/details/hcf690v) | `` | No package | No zip, cab, exe or rar in the item |
| [Hexic for Pocket PC](https://archive.org/details/carbonated-games-hexic) | `` | No package | No zip, cab, exe or rar in the item |
| [Honeywell Dolphin 70e software & firmware](https://archive.org/details/honeywell-dolphin-70e-black-sw-fw) | `` | No package | No zip, cab, exe or rar in the item |
| [HP F1869-80001 Pocket Camera CD-ROM](https://archive.org/details/hp-pocket-camera-cd-rom) | `` | No package | No zip, cab, exe or rar in the item |
| [HP Foldable Keyboard for the iPAQ Pocket PC](https://archive.org/details/gmcd-1003) | `` | No package | No zip, cab, exe or rar in the item |
| [Hp I PAQ Pocket PC Companion CD](https://archive.org/details/hp-i-paq-pocket-pc-companion-cd) | `` | No package | No zip, cab, exe or rar in the item |
| [HP iPAQ 300 Series Travel Companion Getting Started CD](https://archive.org/details/hp-ipaq-300-series-travel-companion-getting-started-cd) | `` | No package | No zip, cab, exe or rar in the item |
| [HP iPAQ H2200 Pocket PC Companion CD](https://archive.org/details/office-10_202112) | `` | No package | No zip, cab, exe or rar in the item |
| [HP iPaq h5100 and h5500 series Companion CD](https://archive.org/details/hp-ipaq-h5550-companion-cd) | `` | No package | No zip, cab, exe or rar in the item |
| [HP iPAQ hw6900 Mobile Messenger](https://archive.org/details/hp-ipaq-hw6900-software) | `` | No package | No zip, cab, exe or rar in the item |
| [HP iPAQ hx2000 Pocket PC series companion CD](https://archive.org/details/office10_ipaq_hx2000) | `` | No package | No zip, cab, exe or rar in the item |
| [HP IPAQ hx2190b Recovery & Synchronization CD](https://archive.org/details/hp-ipaq-hx-2190b-recovery-synchronization-cd) | `` | No package | No zip, cab, exe or rar in the item |
| [HP iPAQ HX4700 ROM Pack](https://archive.org/details/hx4700_ROM) | `` | No package | No zip, cab, exe or rar in the item |
| [HP iPAQ PhotoSmart Mobile Camera software CD](https://archive.org/details/hpcamera_202110) | `` | No package | No zip, cab, exe or rar in the item |
| [HP iPAQ Pocket PC CD h5400 Series (한국어)](https://archive.org/details/hp-i-paq-pocket-pc-cd-h-5400-series-korean) | `` | No package | No zip, cab, exe or rar in the item |
| [HP iPAQ Pocket PC Companion CD (Disc 1715) for iPAQ h4150/h4155 – Windows Mobile 2003 SE Software Bundle](https://archive.org/details/hp-i-paq-h-4150-h-4155) | `` | No package | No zip, cab, exe or rar in the item |
| [HP iPAQ Pocket PC Companion CD h5400 (한국어)](https://archive.org/details/hp-i-paq-h-5400-companion-cd-korean) | `` | No package | No zip, cab, exe or rar in the item |
| [HP iPAQ Pocket PC h1900 series companion CD](https://archive.org/details/hp-ipaq-h1900-cd) | `` | No package | No zip, cab, exe or rar in the item |
| [HP iPAQ Pocket PC h5400 series Companion CD](https://archive.org/details/pocketpc_202511) | `` | No package | No zip, cab, exe or rar in the item |
| [HP iPaq rx1950 get started disc](https://archive.org/details/ipaqgetstartedrx1950) | `` | No package | No zip, cab, exe or rar in the item |
| [HP Jornada 520 Series CD-ROM (USA) (En,Fr,De)](https://archive.org/details/Nova_HPJornada520_USA) | `[LOGS] HP Jornada 520 Series CD-ROM (USA).zip` | Download failed | Download from archive.org failed |
| [HP Jornada 520/540 Solutions CD](https://archive.org/details/jornada_202109) | `` | No package | No zip, cab, exe or rar in the item |
| [HP Jornada 560 Series PDA Pocket PC Companion](https://archive.org/details/office-10_202601) | `` | No package | No zip, cab, exe or rar in the item |
| [HP Jornada 560 series Pocket Keyboard Driver CD](https://archive.org/details/hp_pocketKB104) | `` | No package | No zip, cab, exe or rar in the item |
| [Hp Pocket Camera For Hp Jornada Pocket Pc](https://archive.org/details/hp-pocket-camera-for-hp-jornada-pocket-pc) | `` | No package | No zip, cab, exe or rar in the item |
| [hpc.ru PDA software](https://archive.org/details/hpc_ru) | `` | No package | No zip, cab, exe or rar in the item |
| [HTC HD2 English Windows Mobile 6.2](https://archive.org/details/htc-h-2) | `` | No package | No zip, cab, exe or rar in the item |
| [HTC HD2 Leo Stock ROM Firmware German Deutsch](https://archive.org/details/ruu-leo-s-htc-ger-3.14.407.2-radio-15.42.50.11-u-2.15.50.14-leo-s-ship) | `` | No package | No zip, cab, exe or rar in the item |
| [HTC HD2 Official RUU Files](https://archive.org/details/htc-hd2-official-ruu-files) | `` | No package | No zip, cab, exe or rar in the item |
| [HTC Hermes 8525 ( Updated Sept 2025)](https://archive.org/details/htc-hermes-8525-updated-sept-2025) | `` | No package | No zip, cab, exe or rar in the item |
| [HTC Pure - v2.31.502.4 WWE ROM Dump](https://archive.org/details/htc-pure-2.31.502.4-wwe-rom-dump) | `` | No package | No zip, cab, exe or rar in the item |
| [HTC S730 - v1.31.405.1 ROM Dump](https://archive.org/details/htc-s730-1.31.405.1-rom-dump) | `` | No package | No zip, cab, exe or rar in the item |
| [HTC Sprint PPC-6601](https://archive.org/details/htc_sprint_PPC-6601) | `` | No package | No zip, cab, exe or rar in the item |
| [HTC Touch - Onyx 4 (Windows Mobile 6.1 ROMs)](https://archive.org/details/onyx_ult_4.11) | `` | No package | No zip, cab, exe or rar in the item |
| [HTC Touch - Onyx 6 Release 14 (Windows Mobile 6.5.5 ROM)](https://archive.org/details/onyx_6_release14) | `` | No package | No zip, cab, exe or rar in the item |
| [HTC Touch Diamond Companion Discs](https://archive.org/details/htc-touch-diamond-companion-discs) | `` | No package | No zip, cab, exe or rar in the item |
| [HTC Touch Diamond SPL Unlock+ Stock ROM](https://archive.org/details/htc-touch-diamond-spl-unlock-stock-rom) | `` | No package | No zip, cab, exe or rar in the item |
| [HTC Touch Elf/Elfin Windows Mobile phone - tools and ROMs](https://archive.org/details/htc-touch-elf-and-elfin-package) | `` | No package | No zip, cab, exe or rar in the item |
| [HTC Touch Pro ("Rafael") Prototype dumps](https://archive.org/details/htc-raphael-proto-dumps) | `` | No package | No zip, cab, exe or rar in the item |
| [HTC Touch2 (Codename: Mega) Stock ROMs (RUUs)](https://archive.org/details/touch2ranranruu) | `` | No package | No zip, cab, exe or rar in the item |
| [HX2000 ROM Pack](https://archive.org/details/hx2000) | `` | No package | No zip, cab, exe or rar in the item |
| [I GO 8 For Windows Mobile](https://archive.org/details/i-go-8-for-windows-mobile) | `` | No package | No zip, cab, exe or rar in the item |
| [iBIZ Pocket Radio for PocketPC CompactFlash FM Radio Card Drivers](https://archive.org/details/ibiz-pocket-radio-compactflash-pocketpc-MIPS-ARM) | `` | No package | No zip, cab, exe or rar in the item |
| [Ice Cream Tostidroid V11.0 Normal](https://archive.org/details/icecreamtostidroidv11.0normal) | `` | No package | No zip, cab, exe or rar in the item |
| [iGo-Stowaway-Keyboard-Drivers](https://archive.org/details/iGo-Stowaway-Keyboard) | `` | No package | No zip, cab, exe or rar in the item |
| [Intel Create & Share Software for the Intel Pocket PC Camera](https://archive.org/details/INTELPOCKETCAMCD) | `` | No package | No zip, cab, exe or rar in the item |
| [IntelliGolf Birdie edition / Pocket PC](https://archive.org/details/tucows_287184_IntelliGolf_Birdie_edition_Pocket_PC) | `igtbrdiece.exe` | Download failed | Download from archive.org failed |
| [iPAQ 214 Support Cdrom](https://archive.org/details/iPAQ-214-support-cdrom) | `` | No package | No zip, cab, exe or rar in the item |
| [iPAQ Pocket PC 2002 Service Pack 3](https://archive.org/details/pocket2002sp3) | `` | No package | No zip, cab, exe or rar in the item |
| [iPAQ Pocket PC Rom Upgrade 1.77](https://archive.org/details/ipaq_ROM1.77) | `` | No package | No zip, cab, exe or rar in the item |
| [iPAQ_210_ROMs](https://archive.org/details/iPAQ_210_ROMs) | `` | No package | No zip, cab, exe or rar in the item |
| [jUMLix](https://archive.org/details/tucows_290352_jUMLix) | `` | No package | No zip, cab, exe or rar in the item |
| [Kritical Klondike](https://archive.org/details/tucows_258626_Kritical_Klondike) | `KKlondike_DEMO_ARM.zip` | Download failed | Download from archive.org failed |
| [Lardian PocketBible Pocket PC 2006 (Extracted)](https://archive.org/details/pocketbible2006) | `PocketBiblePktPC.ARM.cab` | Download failed | Download from archive.org failed |
| [Lardian PocketBible Pocket PC 2006 (Extracted)](https://archive.org/details/pocketbible2006) | `PocketBiblePktPC.ARMWM5.cab` | Download failed | Download from archive.org failed |
| [Largest archive of HTC HD2 stock ROMs (RUU)](https://archive.org/details/htc-hd2-stock-ruu) | `` | No package | No zip, cab, exe or rar in the item |
| [last version of mytube before removal](https://archive.org/details/last-version-of-mytube-before-removal) | `` | No package | No zip, cab, exe or rar in the item |
| [Les meilleurs jeux de reflexion et de hasard](https://archive.org/details/image_20260216_1125) | `` | No package | No zip, cab, exe or rar in the item |
| [LG GW 550 Original](https://archive.org/details/lg-gw-550-original) | `` | No package | No zip, cab, exe or rar in the item |
| [Library of old Pocket PC software (Early-Mid 2000's)](https://archive.org/details/PPCISO2006) | `` | No package | No zip, cab, exe or rar in the item |
| [LoanWhiz](https://archive.org/details/tucows_292479_LoanWhiz) | `` | No package | No zip, cab, exe or rar in the item |
| [lokiwiz](https://archive.org/details/lokiwiz) | `lokiwiz.zip` | Download failed | Download from archive.org failed |
| [MailSorter](https://archive.org/details/tucows_32237_MailSorter) | `MSARM211.zip` | Download failed | Download from archive.org failed |
| [Marc Saltzman's Top 300 Games For The Pocket PC OS](https://archive.org/details/marc-saltzmans-top-300-games-for-the-pocket-pc-os) | `` | No package | No zip, cab, exe or rar in the item |
| [Master Kick 1.6.0.2356 REGGED](https://archive.org/details/master-kick-1.6.0.2356-regged) | `Master Kick 1.6.0.2356 REGGED.zip` | Download failed | Download from archive.org failed |
| [Microsoft ActiveSync 3.1 (USA)](https://archive.org/details/Nova_MSActiveSync3-1_USA) | `` | No package | No zip, cab, exe or rar in the item |
| [Microsoft ActiveSync 4.5](https://archive.org/details/MS_ActiveSync-4) | `` | No package | No zip, cab, exe or rar in the item |
| [Microsoft Cubicle Chaos](https://archive.org/details/microsoft-cubicle-chaos) | `` | No package | No zip, cab, exe or rar in the item |
| [Microsoft Device Emulator Without Saves](https://archive.org/details/microsoft-device-emulator-without-saves) | `` | No package | No zip, cab, exe or rar in the item |
| [Microsoft eMbedded Visual Tools 3.0 - April 2000](https://archive.org/details/ms-embedvtools-30-0400) | `` | No package | No zip, cab, exe or rar in the item |
| [Microsoft Emulator Windows 10 For Mobile 10.0.14393.0](https://archive.org/details/microsoft-emulator-windowws-10-for-mobile-10.0.14393.0) | `` | No package | No zip, cab, exe or rar in the item |
| [Microsoft Entertainment Pack 2.0 for Windows CE (USA)](https://archive.org/details/Nova_MicrosoftEntertainmentPack2WinCE_USA) | `` | No package | No zip, cab, exe or rar in the item |
| [Microsoft Games PocketPak for Windows Mobile/Pocket PC (EN)](https://archive.org/details/microsoft-games-pocket-pak) | `Microsoft Games PocketPak.zip` | Download failed | Download from archive.org failed |
| [Microsoft Minecraft for Windows Mobile Archive](https://archive.org/details/MMFWMA) | `` | No package | No zip, cab, exe or rar in the item |
| [Microsoft Mobile Application Development Toolkit](https://archive.org/details/040916-1916) | `` | No package | No zip, cab, exe or rar in the item |
| [Microsoft Mobile DevCon 2004 Conference DVD](https://archive.org/details/mdc-2004_202504) | `` | No package | No zip, cab, exe or rar in the item |
| [Microsoft Money 2005 for Pocket PC](https://archive.org/details/microsoft-money-2005-for-pocket-pc) | `arm720.cab` | Download failed | Download from archive.org failed |
| [Microsoft Plus! Digital Media Edition (USA)](https://archive.org/details/redump-id-133929) | `` | No package | No zip, cab, exe or rar in the item |
| [Microsoft Pocket PC 2002 Upgrade for the Compaq IPAQ Pocket PC H3600 Series (257344 001)(Compaq)(2001)](https://archive.org/details/Microsoft_Pocket_PC_2002_Upgrade_for_the_Compaq_IPAQ_Pocket_PC_H3600_Series_2573) | `` | No package | No zip, cab, exe or rar in the item |
| [Microsoft Pocket PC 2003 SDK](https://archive.org/details/PocketPC2003SDK) | `` | No package | No zip, cab, exe or rar in the item |
| [Microsoft Pocket PC Information CD version 2 (2001, France)](https://archive.org/details/msft-ppc-fr) | `` | No package | No zip, cab, exe or rar in the item |
| [Microsoft Pocket Streets 2005 for Windows Pocket PC and Smartphone (X10 80572)(2004)](https://archive.org/details/Microsoft_Pocket_Streets_2005_for_Windows_Pocket_PC_and_Smartphone_X10-80572_200) | `` | No package | No zip, cab, exe or rar in the item |
| [Microsoft PocketPC 2002 Companion CD](https://archive.org/details/ppc2002jp) | `` | No package | No zip, cab, exe or rar in the item |
| [Microsoft SDK for Windows CE, Handheld PC Professional Edition 3.0](https://archive.org/details/hpcpro-sdk-usa) | `` | No package | No zip, cab, exe or rar in the item |
| [Microsoft Voice Command V1.0.1](https://archive.org/details/MSVoiceCommand) | `` | No package | No zip, cab, exe or rar in the item |
| [Microsoft Windows Mobile 10 Emulator Image - Build 10586](https://archive.org/details/ms-w10m-emulatorvhd-10586) | `` | No package | No zip, cab, exe or rar in the item |
| [Microsoft Windows Mobile 10 Prerelease Emulator Kits](https://archive.org/details/ms-w10m-emulatorsdk) | `` | No package | No zip, cab, exe or rar in the item |
| [Microsoft Works Suite 2003 CD (OEM)](https://archive.org/details/wksste) | `` | No package | No zip, cab, exe or rar in the item |
| [Microsoft® Windows® Platform SDK for Pocket PC 2000](https://archive.org/details/pocketpc_2000_sdk_web) | `` | No package | No zip, cab, exe or rar in the item |
| [MindSoft Pocket VirusShield](https://archive.org/details/tucows_258609_MindSoft_Pocket_VirusShield) | `` | No package | No zip, cab, exe or rar in the item |
| [Mirror's Edge Mobile (Windows Mobile)](https://archive.org/details/mirrors-edge-mobile) | `` | No package | No zip, cab, exe or rar in the item |
| [Mobi December 2008 Cover DVD](https://archive.org/details/mobi-2008-12-dvd) | `` | No package | No zip, cab, exe or rar in the item |
| [Mobi February 2007 Cover DVD](https://archive.org/details/mobi-2007-02-dvd) | `` | No package | No zip, cab, exe or rar in the item |
| [Mobi February 2008 Cover DVD](https://archive.org/details/mobi-2009-02-dvd) | `` | No package | No zip, cab, exe or rar in the item |
| [Mobi June 2008 Cover DVD](https://archive.org/details/mobi-2008-06-dvd) | `` | No package | No zip, cab, exe or rar in the item |
| [Mobile Computer User Magazine - CD11 - Christmas 2000](https://archive.org/details/mcu-cd-11) | `` | No package | No zip, cab, exe or rar in the item |
| [Mobile Computer User Magazine - CD12 - January 2001](https://archive.org/details/mcu-cd-12) | `` | No package | No zip, cab, exe or rar in the item |
| [Mobile Crossing WayPoint 100 / 200 installation disc](https://archive.org/details/mobile-crossing-waypoint-pocket-pc-100-and-200) | `` | No package | No zip, cab, exe or rar in the item |
| [Mobile Golf Scorer PAR Version](https://archive.org/details/tucows_356722_Mobile_Golf_Scorer_PAR_Version) | `MobileGolfScorerPPC-E.exe` | Download failed | Download from archive.org failed |
| [Motorola - MOTO Q9h Software (2007)](https://archive.org/details/motoq9hgsdx10) | `` | No package | No zip, cab, exe or rar in the item |
| [Motorola MPx200 WM 5.0[RUS] RC3 (Custom ROM)](https://archive.org/details/wm50rusrc3) | `` | No package | No zip, cab, exe or rar in the item |
| [Motorola MPx200 WM06 Black ONE](https://archive.org/details/mpx200wm06blackone) | `` | No package | No zip, cab, exe or rar in the item |
| [Mozaki Blocks for Pocket PC](https://archive.org/details/carbonated-games-mozaki) | `` | No package | No zip, cab, exe or rar in the item |
| [MPx220 1.53 By Hardrom Update](https://archive.org/details/1.53-by-hardrom-update) | `` | No package | No zip, cab, exe or rar in the item |
| [MPx220 Firmware 1.53 Brasil](https://archive.org/details/1.53-gbrasil) | `` | No package | No zip, cab, exe or rar in the item |
| [MSDN Disc 2790 November 2004](https://archive.org/details/msdn-2790) | `` | No package | No zip, cab, exe or rar in the item |
| [MxConverter](https://archive.org/details/tucows_285781_MxConverter) | `mxconv_ppc.zip` | Download failed | Download from archive.org failed |
| [NaviSys AeroMap V2.4](https://archive.org/details/aero-map-v-2) | `` | No package | No zip, cab, exe or rar in the item |
| [NFS Shift (Windows Mobile)](https://archive.org/details/nfss-wm) | `` | No package | No zip, cab, exe or rar in the item |
| [OFFICE 10 (HTC Vox S710 CD - Europe - Français)](https://archive.org/details/office-10_202308) | `` | No package | No zip, cab, exe or rar in the item |
| [Palbum](https://archive.org/details/tucows_234798_Palbum) | `palbum_setup.exe` | Download failed | Download from archive.org failed |
| [Palbum Picture Optimizer](https://archive.org/details/tucows_245210_Palbum_Picture_Optimizer) | `palbumpicop.zip` | Download failed | Download from archive.org failed |
| [Palm Heroes Level Editor](https://archive.org/details/palm-heroes-level-editor) | `` | No package | No zip, cab, exe or rar in the item |
| [Palm Pocket Magazine España CD](https://archive.org/details/palm-pocket-magazine-cd) | `` | No package | No zip, cab, exe or rar in the item |
| [PALM-CAD](https://archive.org/details/tucows_32642_PALM-CAD) | `setuppalmcad.exe` | Download failed | Download from archive.org failed |
| [Palmari Software Library Vol 3](https://archive.org/details/palmari-software-library-vol-3) | `` | No package | No zip, cab, exe or rar in the item |
| [Paragon Software Asus P535 CD](https://archive.org/details/paragon_software_asus_p535) | `` | No package | No zip, cab, exe or rar in the item |
| [Paranoid Android Leo Beta V 2.15](https://archive.org/details/paleobetav2.15) | `` | No package | No zip, cab, exe or rar in the item |
| [PC Team Hors Série 03](https://archive.org/details/pcteam-hs-03) | `` | No package | No zip, cab, exe or rar in the item |
| [PDA & Mobiles CD ROM 03](https://archive.org/details/pda-et-mobiles-03) | `` | No package | No zip, cab, exe or rar in the item |
| [PDA & Mobiles CD ROM 04](https://archive.org/details/pda-et-mobiles-04) | `` | No package | No zip, cab, exe or rar in the item |
| [PDA Essential Issue 57](https://archive.org/details/pda-essential-issue-57) | `` | No package | No zip, cab, exe or rar in the item |
| [PDA Pratique CD-ROM 02](https://archive.org/details/pda_pratique_numero02) | `` | No package | No zip, cab, exe or rar in the item |
| [PDA Pratique CD-ROM 06](https://archive.org/details/pda_pratique_numero06) | `` | No package | No zip, cab, exe or rar in the item |
| [PDA Pratique CD-ROM 07](https://archive.org/details/pda_pratique_numero07) | `` | No package | No zip, cab, exe or rar in the item |
| [PDA Pratique CD-ROM 08](https://archive.org/details/pda_pratique_numero08) | `` | No package | No zip, cab, exe or rar in the item |
| [PDA Pratique CD-ROM 09](https://archive.org/details/pda_pratique_numero09) | `` | No package | No zip, cab, exe or rar in the item |
| [PDA Pratique CD-ROM 13](https://archive.org/details/pda_pratique_numero13) | `` | No package | No zip, cab, exe or rar in the item |
| [PDA Pratique CD-ROM 14](https://archive.org/details/pda_pratique_numero14) | `` | No package | No zip, cab, exe or rar in the item |
| [PDA Pratique CD-ROM 15](https://archive.org/details/pda_pratique_numero15) | `` | No package | No zip, cab, exe or rar in the item |
| [PDA Pratique CD-ROM 17](https://archive.org/details/pda_pratique_numero17CD2) | `` | No package | No zip, cab, exe or rar in the item |
| [PDA Pratique CD-ROM 19](https://archive.org/details/pda_pratique_numero19CD2) | `` | No package | No zip, cab, exe or rar in the item |
| [PDA Pratique CD-ROM 27](https://archive.org/details/pda_pratique_numero27CD1) | `` | No package | No zip, cab, exe or rar in the item |
| [PDA Pratique CD-ROM Hors Série 01 bis](https://archive.org/details/pda_pratique_numerohs01) | `` | No package | No zip, cab, exe or rar in the item |
| [PDA Pratique CDROM Hors Série 01](https://archive.org/details/pda_pratique_numerohs01cd1) | `` | No package | No zip, cab, exe or rar in the item |
| [Pendulum](https://archive.org/details/tucows_335975_Pendulum) | `pendulum.exe` | Download failed | Download from archive.org failed |
| [Pharos Ostia 5.01 030603](https://archive.org/details/ostia-5.01-030603) | `` | No package | No zip, cab, exe or rar in the item |
| [Pharos OSTIA GPS 6.0 for Pocket PC](https://archive.org/details/Ostia6.0) | `` | No package | No zip, cab, exe or rar in the item |
| [Phone Roms HTC Hermes SPV 3100](https://archive.org/details/phone-roms-spv-3100.7z) | `` | No package | No zip, cab, exe or rar in the item |
| [Pocket Basketball - 2002](https://archive.org/details/tucows_224453_Pocket_Basketball_-_2002) | `pkBasketball.exe` | Download failed | Download from archive.org failed |
| [Pocket BizCalc Pocket PC](https://archive.org/details/Pocket-BizCalc-1999-02-22-ce-ppc) | `pbizcalc.mips.CAB` | Download failed | Download from archive.org failed |
| [Pocket Lemmings](https://archive.org/details/pocket-lemmings) | `PocketLemmings.cab` | Download failed | Download from archive.org failed |
| [Pocket Linux 1.0](https://archive.org/details/PocketLinux1.0) | `` | No package | No zip, cab, exe or rar in the item |
| [Pocket PC - Redump.info](https://archive.org/details/pocket_pc) | `Pocket Flash Format (Japan).zip` | Download failed | Download from archive.org failed |
| [Pocket PC 2003 Upgrade For iPAQ 3970, 3975, 3950, 3955, 3749775](https://archive.org/details/PocketPC2003UpgradeForIPAQ3970397539503955.3749775.TPB2) | `` | No package | No zip, cab, exe or rar in the item |
| [Pocket PC 2003 Upgrade For iPAQ 3970, 3975, 3950, 3955, 3749775 (reupload)](https://archive.org/details/pocket-pc-2003-upgrade-for-i-paq-3970-3975-3950-3955) | `` | No package | No zip, cab, exe or rar in the item |
| [Pocket PC 2003 Upgrade for the iPaq h3900 series](https://archive.org/details/ipaq-h3900-series-pocket-pc-2003-upgrade) | `` | No package | No zip, cab, exe or rar in the item |
| [Pocket PC 2003 w/Outlook 2002](https://archive.org/details/PocketPC2003WOutlook2002) | `` | No package | No zip, cab, exe or rar in the item |
| [Pocket PC 4.0 (DISC-2011)](https://archive.org/details/pocket-pc-4.0) | `` | No package | No zip, cab, exe or rar in the item |
| [Pocket PC and Windows Mobile/CE software, roms and mixed stuff.](https://archive.org/details/packs_202105) | `CorePlayer PPC 1.3.6 full.rar` | Download failed | Download from archive.org failed |
| [Pocket PC Application DVD](https://archive.org/details/PocketPCDump) | `` | No package | No zip, cab, exe or rar in the item |
| [Pocket Pc Emulator native frames per second hack](https://archive.org/details/pocket_pc_emulator) | `vs_emulator_x64_vista.exe` | Download failed | Download from archive.org failed |
| [Pocket PC Films: Kids Picks](https://archive.org/details/kids-picks) | `` | No package | No zip, cab, exe or rar in the item |
| [Pocket PC Game Programming](https://archive.org/details/pocketpcgameprogramming) | `` | No package | No zip, cab, exe or rar in the item |
| [Pocket PC Games](https://archive.org/details/pocket-pc-games) | `` | No package | No zip, cab, exe or rar in the item |
| [Pocket PC MDPPC 150 Companion CD](https://archive.org/details/40006052) | `` | No package | No zip, cab, exe or rar in the item |
| [Pocket PC port of the Simon Tatham's Portable Puzzle Collection](https://archive.org/details/pocket-pc-port-of-the-simon-tathams-portable-puzzle-collection) | `puzzles-r7415-2007-03-29.ARMV4.CAB` | Download failed | Download from archive.org failed |
| [Pocket PC Software (Curated)](https://archive.org/details/pocket-pc-software) | `` | No package | No zip, cab, exe or rar in the item |
| [Pocket Piano PPC ARM Full](https://archive.org/details/pocket-piano.-ppc-arm-full) | `` | No package | No zip, cab, exe or rar in the item |
| [Pocket Punchclock](https://archive.org/details/tucows_286263_Pocket_Punchclock) | `PocketPunchclockSetup.exe` | Download failed | Download from archive.org failed |
| [Pocket Sudoku Pocket PC / Windows Mobile](https://archive.org/details/pocket-sudoku-pocket-pc) | `` | No package | No zip, cab, exe or rar in the item |
| [Pocket Tablet](https://archive.org/details/tucows_236182_Pocket_Tablet) | `` | No package | No zip, cab, exe or rar in the item |
| [pocket Theme Manager](https://archive.org/details/tucows_236688_pocket_Theme_Manager) | `pocketThemeManager_ENG.exe` | Download failed | Download from archive.org failed |
| [PocketAdmin](https://archive.org/details/tucows_359245_PocketAdmin) | `` | No package | No zip, cab, exe or rar in the item |
| [PocketAdmin](https://archive.org/details/tucows_359248_PocketAdmin) | `` | No package | No zip, cab, exe or rar in the item |
| [PocketBar 1.2](https://archive.org/details/pocketbar12) | `pocketbar_v1.rar` | Download failed | Download from archive.org failed |
| [PocketDBA](https://archive.org/details/tucows_359244_PocketDBA) | `` | No package | No zip, cab, exe or rar in the item |
| [PocketDBA](https://archive.org/details/tucows_359247_PocketDBA) | `` | No package | No zip, cab, exe or rar in the item |
| [PocketHandy 2002](https://archive.org/details/tucows_259280_PocketHandy_2002) | `pockethandy_demo.zip` | Download failed | Download from archive.org failed |
| [pocketpc. 2002.rom.upgrade.ipaq. 3600.rar](https://archive.org/details/pocketpc.2002.rom.upgrade.ipaq.3600.rar) | `` | No package | No zip, cab, exe or rar in the item |
| [PPC Pocket PC Windows Mobile 2003 SE WM 6.1 WM 6.5 Ken Garage Pack Apps&Games](https://archive.org/details/ppc-wm-2003-se-wm-6.1-wm-6.5-ken-garage-pack-apps-games) | `` | No package | No zip, cab, exe or rar in the item |
| [PPC Pocket PC WM2003 WM5 PDA games and software](https://archive.org/details/ppc_20250107) | `` | No package | No zip, cab, exe or rar in the item |
| [PPC v2 UK](https://archive.org/details/PPCv2) | `` | No package | No zip, cab, exe or rar in the item |
| [ppcsoftware](https://archive.org/details/ppcsoftware) | `` | No package | No zip, cab, exe or rar in the item |
| [Practical Programming in Tcl and Tk (4th Edition) CD-ROM](https://archive.org/details/tclbook4) | `` | No package | No zip, cab, exe or rar in the item |
| [PrintBoy CE](https://archive.org/details/tucows_244924_PrintBoy_CE) | `PrintBoyCE_Install.zip` | Download failed | Download from archive.org failed |
| [Project Astoria For Windows Mobile with lumia flashtool](https://archive.org/details/project-astoria-for-windows-mobile-files.-7z) | `` | No package | No zip, cab, exe or rar in the item |
| [PTab Spreadsheet](https://archive.org/details/tucows_32606_PTab_Spreadsheet) | `PTabHPCTrialSetup.exe` | Download failed | Download from archive.org failed |
| [QuickCalendar 2002](https://archive.org/details/tucows_243720_QuickCalendar_2002) | `quickcalendar_2002.zip` | Download failed | Download from archive.org failed |
| [QuickMinutes Pro](https://archive.org/details/tucows_283565_QuickMinutes_Pro) | `` | No package | No zip, cab, exe or rar in the item |
| [Raphael HTC RUS 5.05.411.3 By Football](https://archive.org/details/raphael-htc-rus-5.05.411.3-by-football) | `` | No package | No zip, cab, exe or rar in the item |
| [Rayman Pocket (Pocket PC) + Emulator](https://archive.org/details/rayman-pocket-pocket-pc-emulator) | `` | No package | No zip, cab, exe or rar in the item |
| [Rayman Ultimate (Pocket PC) + Emulator](https://archive.org/details/rayman-ultimate-pocket-pc-emulator) | `` | No package | No zip, cab, exe or rar in the item |
| [Resco Picture Viewer](https://archive.org/details/tucows_153953_Resco_Picture_Viewer) | `picview_p.exe` | Download failed | Download from archive.org failed |
| [riteMail](https://archive.org/details/tucows_205191_riteMail) | `` | No package | No zip, cab, exe or rar in the item |
| [ROMs for HTC HD2](https://archive.org/details/hd-2-roms) | `` | No package | No zip, cab, exe or rar in the item |
| [ROUTE 66 NAVIGATE ESPAÑA Y PORTUGAL](https://archive.org/details/route-66-navigate-espan-a-y-portugal) | `` | No package | No zip, cab, exe or rar in the item |
| [ROUTE 66 Sync & Backup CD-ROM (2008)](https://archive.org/details/route66-sync-backup) | `` | No package | No zip, cab, exe or rar in the item |
| [RPS Fract](https://archive.org/details/tucows_241197_RPS_Fract) | `RPSFract.zip` | Download failed | Download from archive.org failed |
| [RSPB Birds Of Britain & Ireland ( UK) Patch`s](https://archive.org/details/rspb-birds-of-britain-ireland-uk-patchs) | `RSPB Birds of Britain & Ireland (UK) Patch`s.zip` | Download failed | Download from archive.org failed |
| [Samsung Omnia i900 ROM's and apps](https://archive.org/details/samsung-omnia-i-900-1731408742717) | `` | No package | No zip, cab, exe or rar in the item |
| [Samsung Omnia II Verizon software disc v7.0.12-2384a](https://archive.org/details/samsung_omnia_i920_verizon_software) | `` | No package | No zip, cab, exe or rar in the item |
| [Samsung SGH-I637 Getting Started Disc (2009)](https://archive.org/details/sghi637) | `` | No package | No zip, cab, exe or rar in the item |
| [Scrabble and Monopoly (331-0803-10)](https://archive.org/details/Handmark) | `` | No package | No zip, cab, exe or rar in the item |
| [ScreenShooter](https://archive.org/details/tucows_259112_ScreenShooter) | `screenshooter-arm-trial.exe` | Download failed | Download from archive.org failed |
| [SHARP Zaurus Software Collection](https://archive.org/details/sharp-zaurus-collection) | `` | No package | No zip, cab, exe or rar in the item |
| [shouji.com.cn Windows Mobile Games (1)](https://archive.org/details/shouji.com.cn-winmo-folder-1.7z) | `` | No package | No zip, cab, exe or rar in the item |
| [Siemens SX45 Product CD v1.0](https://archive.org/details/Siemens-SX45-Product-CD-v1.0) | `` | No package | No zip, cab, exe or rar in the item |
| [SKTools 4.4.6.2 WM 2003(SE), WM5 Pocket PC, WM6,6.1 Classic and](https://archive.org/details/SKTools4.4.6.2WM2003SEWM5PocketPCWM66.1Classic) | `` | No package | No zip, cab, exe or rar in the item |
| [Skype v 3.0.0.256 for windows mobile .cab](https://archive.org/details/SkypeV3.0.0.256WM) | `` | No package | No zip, cab, exe or rar in the item |
| [Smartphone.ua downloads section scrape](https://archive.org/details/smartphone_ua_scrape) | `` | No package | No zip, cab, exe or rar in the item |
| [Socket Rugged CF 10/100 Installation Disc](https://archive.org/details/sckt-10-100-164-b) | `` | No package | No zip, cab, exe or rar in the item |
| [Softmaker Office 2008 Pocket PC Beta (ofpw08beta)](https://archive.org/details/softmakeroffice2008pocketpcofpw08beta) | `` | No package | No zip, cab, exe or rar in the item |
| [Sony Ericsson Xperia X1 UK GENERIC ROM](https://archive.org/details/sw-x-1-generic-uk-1213-6450-r-3-aa-010) | `` | No package | No zip, cab, exe or rar in the item |
| [Sony Ericsson Xperia X1 Windows Mobile 6.5 port [SEX1 6.5 KOVSKY]](https://archive.org/details/sex-1-6.5-kovsky) | `` | No package | No zip, cab, exe or rar in the item |
| [Spb Arkaball](https://archive.org/details/tucows_302509_Spb_Arkaball) | `Arkaball_setup.exe` | Download failed | Download from archive.org failed |
| [SpreadCE 3.14 2023-06-10](https://archive.org/details/spreadce-3.14-2023-06-10) | `spreppcm.zip` | Download failed | Download from archive.org failed |
| [Sprint HTC Touch Pro 2 Software CD (P/N: 69H10135-00M Ver. A)](https://archive.org/details/htctouchpro-2) | `` | No package | No zip, cab, exe or rar in the item |
| [SPV M2000 RUU V 2.5 Orange UK V 5.40.1.179.ship](https://archive.org/details/ruu-v-2.5-orange-uk-v-5.40.1.179.ship) | `` | No package | No zip, cab, exe or rar in the item |
| [Street Atlas USA 2008 Plus](https://archive.org/details/sa2008plus) | `` | No package | No zip, cab, exe or rar in the item |
| [Sunnysoft InterWrite 6.2P](https://archive.org/details/tucows_260396_Sunnysoft_InterWrite_6.2P) | `Sunnysoft_InterWrite_62P_Demo.zip` | Download failed | Download from archive.org failed |
| [Super Elemental](https://archive.org/details/tucows_293118_Super_Elemental) | `SuperElementalDemo.exe` | Download failed | Download from archive.org failed |
| [SyncTalk FX](https://archive.org/details/tucows_219052_SyncTalk_FX) | `SyncTalk_PocketPC.exe` | Download failed | Download from archive.org failed |
| [SysOn CF Memory GPS Application CD](https://archive.org/details/syson-cf-memory-gps-application-cd) | `` | No package | No zip, cab, exe or rar in the item |
| [T-Mobile MDA Touch Plus - Getting Started Disc [69H10089-27M] (Version A, 12/2007)](https://archive.org/details/t-mobile-mda-touch-plus-getting-started-disc-69h10089-27m) | `` | No package | No zip, cab, exe or rar in the item |
| [TDT 2005 Disk 30: Windows Mobile](https://archive.org/details/tdt2005) | `` | No package | No zip, cab, exe or rar in the item |
| [Tested software for Windows Mobile 2003](https://archive.org/details/ppc-2003) | `PocketNester/pnsetup_arm.exe` | Download failed | Download from archive.org failed |
| [Tetris classic game pak](https://archive.org/details/tetris_20260724) | `` | No package | No zip, cab, exe or rar in the item |
| [The Core Pocket Media Player](https://archive.org/details/the-core-pocket-media-player) | `The Core Pocket Media Player.cab` | Download failed | Download from archive.org failed |
| [The Silent Age 2015.508.834.2193 appxbundle](https://archive.org/details/silent-age-2015.508.834.2193) | `` | No package | No zip, cab, exe or rar in the item |
| [The Silent Age 2015.508.834.2193 appxbundle for Windows Phone/Mobile](https://archive.org/details/silent-age-2015.508.834.2193_202601) | `` | No package | No zip, cab, exe or rar in the item |
| [Theme Generator v1.0 for Smartphone 2002](https://archive.org/details/theme-generator-smartphone) | `` | No package | No zip, cab, exe or rar in the item |
| [TicTacToe for 2 players](https://archive.org/details/tucows_238638_TicTacToe_for_2_players) | `t3.zip` | Download failed | Download from archive.org failed |
| [Tipster for Palm-Size PC (Extracted)](https://archive.org/details/Tipster.mips.sh3) | `Tipster.mips.CAB` | Download failed | Download from archive.org failed |
| [Tipster for Palm-Size PC (Extracted)](https://archive.org/details/Tipster.mips.sh3) | `Tipster.sh3.CAB` | Download failed | Download from archive.org failed |
| [Tipy a triky pro Pocket PC](https://archive.org/details/k-1054) | `` | No package | No zip, cab, exe or rar in the item |
| [Tom Tom City Maps Paris](https://archive.org/details/tom-tom-city-maps-paris) | `` | No package | No zip, cab, exe or rar in the item |
| [Tomtom Citymaps 3.0 (2001)](https://archive.org/details/tomtom-citymaps-2001) | `` | No package | No zip, cab, exe or rar in the item |
| [Tomtom Citymaps 3.0a (2003)](https://archive.org/details/tomtom-citymaps-2003) | `` | No package | No zip, cab, exe or rar in the item |
| [Tomtom Routeplanner Europe](https://archive.org/details/tomtom-routeplanner-europe) | `` | No package | No zip, cab, exe or rar in the item |
| [Tony Hawk's Pro Skater 2 for Pocket PCs](https://archive.org/details/thps2_202012) | `` | No package | No zip, cab, exe or rar in the item |
| [Toshiba e330 Series Pocket PC 2002 Companion CD-ROM](https://archive.org/details/pocketpc_202607) | `` | No package | No zip, cab, exe or rar in the item |
| [TOSHIBA PORTEGE G900 Getting Started Disc for G900](https://archive.org/details/portege-g900-disc1) | `` | No package | No zip, cab, exe or rar in the item |
| [Touch Keyboard](https://archive.org/details/tucows_280205_Touch_Keyboard) | `TouchKeyboard103Setup.exe` | Download failed | Download from archive.org failed |
| [Trivial Pursuit - Windows CE Edition (USA)](https://archive.org/details/redump-id-132589) | `` | No package | No zip, cab, exe or rar in the item |
| [TrueTerm CE Culinary Dictionary English/Swedish](https://archive.org/details/tucows_180992_TrueTerm_CE_Culinary_Dictionary_English_Swedish) | `TTWinCEEnglishSwedishEnglish.zip` | Download failed | Download from archive.org failed |
| [Universo PC #1 (CD Software)](https://archive.org/details/universo-pc-1) | `` | No package | No zip, cab, exe or rar in the item |
| [upgrade.hp. 5400.series.to.pocket.pc. 2003](https://archive.org/details/upgrade.hp.5400.series.to.pocket.pc.2003) | `` | No package | No zip, cab, exe or rar in the item |
| [User Manual CD for Samsung Omnia II](https://archive.org/details/samsung_omnia_i920_manual) | `` | No package | No zip, cab, exe or rar in the item |
| [Velo Performance Accelerator](https://archive.org/details/tucows_32558_Velo_Performance_Accelerator) | `VeloTrbo.exe` | Download failed | Download from archive.org failed |
| [ViewSonic Pocket PC V37 Companion CD (ViewSonic) (2003)](https://archive.org/details/ViewSonic_Pocket_PC_V37_Companion_CD_ViewSonic_2003) | `` | No package | No zip, cab, exe or rar in the item |
| [Visual HelpStudio PocketHelp](https://archive.org/details/tucows_167358_Visual_HelpStudio_PocketHelp) | `visualhelpstudioPocketHelp.exe` | Download failed | Download from archive.org failed |
| [Visual Studio 6 For Windows CE](https://archive.org/details/visual-studio-6-for-windows-ce) | `` | No package | No zip, cab, exe or rar in the item |
| [vxFtp](https://archive.org/details/tucows_32228_vxFtp) | `vxFtp.zip` | Download failed | Download from archive.org failed |
| [vxFtpSrv](https://archive.org/details/tucows_241491_vxFtpSrv) | `vxFtpSrv.zip` | Download failed | Download from archive.org failed |
| [WebCopier](https://archive.org/details/tucows_226617_WebCopier) | `wcopier_pocketpc.exe` | Download failed | Download from archive.org failed |
| [Windows 10 Mobile 10.0.15254.1 Emulator](https://archive.org/details/windows-10-mobile-10.0.15254.1-emulator.-7znbab) | `` | No package | No zip, cab, exe or rar in the item |
| [Windows 10 Mobile build 16235 - ARMfre Multilanguage (MAK)](https://archive.org/details/10.0.16235.1000.rs-onecore-wsd-wufb.-170629-1700.7z) | `` | No package | No zip, cab, exe or rar in the item |
| [Windows 10 Mobile build 16241](https://archive.org/details/wm16241) | `` | No package | No zip, cab, exe or rar in the item |
| [Windows 10 Mobile v1607, build 14347.1000 (RS1_RELEASE) Mobile Adaptation Kit with debugging symbol sets - Multilanguage (Rest of World/ROW)](https://archive.org/details/10.0.14347.1000.rs1_release.160513-1723_MobileAdaptationKit) | `` | No package | No zip, cab, exe or rar in the item |
| [Windows Active Sync for MIO A201 devices](https://archive.org/details/windows-active-sync) | `` | No package | No zip, cab, exe or rar in the item |
| [Windows CE and Mobile ROMs Collection](https://archive.org/details/winceroms) | `` | No package | No zip, cab, exe or rar in the item |
| [Windows CE Deutsch / German X03-59289 DE](https://archive.org/details/windows-ce-german) | `` | No package | No zip, cab, exe or rar in the item |
| [Windows CE Emulators](https://archive.org/details/win-ce-6-emulator_202201) | `` | No package | No zip, cab, exe or rar in the item |
| [Windows CE Freeware and Shareware CD by ROMMAN](https://archive.org/details/windows-ce-freeware-and-shareware-cd-by-romman) | `` | No package | No zip, cab, exe or rar in the item |
| [Windows CE Services 2.1 Sync CD and Value Added Software](https://archive.org/details/wincesoftware) | `` | No package | No zip, cab, exe or rar in the item |
| [Windows Device Recovery Tool Offline Installer](https://archive.org/details/wdrt-offline-installer-v-2.1.7z) | `` | No package | No zip, cab, exe or rar in the item |
| [Windows Mobile 10 Retail Signed FFUs](https://archive.org/details/W10M_FFUs) | `` | No package | No zip, cab, exe or rar in the item |
| [Windows Mobile 10 Update Packages](https://archive.org/details/W10M_Cabs) | `` | No package | No zip, cab, exe or rar in the item |
| [Windows Mobile 2002 & 2003 x86 on qemu 0.14.0 rc2 and qemu 0.15.1](https://archive.org/details/qemu-0.14.0-rc2-pocketpc.7z) | `` | No package | No zip, cab, exe or rar in the item |
| [Windows Mobile 2003 build 13126 (Smartphone, x86, VMDK)](https://archive.org/details/smartphone-2003-build-13126) | `` | No package | No zip, cab, exe or rar in the item |
| [Windows Mobile 2003 Second Edition Emulator Images for Smartphone - All Languages](https://archive.org/details/WM2003SESmartphoneEmulator) | `` | No package | No zip, cab, exe or rar in the item |
| [Windows Mobile 5.0 Pocket PC SDK](https://archive.org/details/windows-mobile-5.0-pocket-pc-sdk_202305) | `` | No package | No zip, cab, exe or rar in the item |
| [Windows Mobile 5.0 Pocket PC SDK And Emulator](https://archive.org/details/WindowsMobile5.0PocketPCSDKAndEmulator) | `` | No package | No zip, cab, exe or rar in the item |
| [Windows Mobile 5.0, 6.1, 6.5 Pocket PC SDK](https://archive.org/details/windows-mobile-5.0-pocket-pc-sdk) | `` | No package | No zip, cab, exe or rar in the item |
| [Windows Mobile 6 Localized Emulator Images](https://archive.org/details/WM6LocalizedEmulatorImages) | `` | No package | No zip, cab, exe or rar in the item |
| [Windows Mobile 6 Localized Images (BIN)](https://archive.org/details/wm6-locale) | `` | No package | No zip, cab, exe or rar in the item |
| [Windows Mobile 6 Standard and Professional SDK Refresh](https://archive.org/details/windowsmobile6sdkrefresh) | `` | No package | No zip, cab, exe or rar in the item |
| [Windows Mobile 6.1.4 build 20758 (HTC Universal ROM)](https://archive.org/details/wm-6.1-ce-os-5.2.20758-build-20758.1.4.1-for-universal) | `` | No package | No zip, cab, exe or rar in the item |
| [Windows Mobile 6.1.4 Emulator Images](https://archive.org/details/WM614Emulator) | `` | No package | No zip, cab, exe or rar in the item |
| [Windows Mobile 6.2 x86 Ukranian](https://archive.org/details/Windows-mobile-6.2-x86-ukranian) | `` | No package | No zip, cab, exe or rar in the item |
| [Windows Mobile 6.5 Professional Developer Tool Kit ( USA)](https://archive.org/details/windows-mobile-6.5-professional-developer-tool-kit-usa_202410) | `` | No package | No zip, cab, exe or rar in the item |
| [Windows Mobile 6.5.1 CE 5.2.23063](https://archive.org/details/windows-mobile-6.5.1-ce-5.2.23063) | `` | No package | No zip, cab, exe or rar in the item |
| [Windows Mobile 6.5.3 build 23518 (Acer N311 ROM, Classic SKU)](https://archive.org/details/acer-n-311-wm-6-5-23518-wwe-2010-02-05) | `` | No package | No zip, cab, exe or rar in the item |
| [Windows Mobile 6.5.3 build 29315 (HTC HD2 ROM)](https://archive.org/details/artemis-v-740-0409-wwe.-7z) | `` | No package | No zip, cab, exe or rar in the item |
| [Windows Mobile Developer Resource Kit (2004) [098-99638]](https://archive.org/details/windows-mobile-dev-reskit) | `` | No package | No zip, cab, exe or rar in the item |
| [Windows Mobile Emulation Images Archive](https://archive.org/details/windows-mobile-emulation-images-archive.-7z) | `` | No package | No zip, cab, exe or rar in the item |
| [Windows Mobile Emulator Images Archive](https://archive.org/details/wm-emu-imgs) | `` | No package | No zip, cab, exe or rar in the item |
| [Windows Mobile ROM collection](https://archive.org/details/ROM_collection_Mobile) | `` | No package | No zip, cab, exe or rar in the item |
| [Windows Mobile SDKs (collection)](https://archive.org/details/WMSDK) | `` | No package | No zip, cab, exe or rar in the item |
| [Windows Mobile/Pocket PC SDKs](https://archive.org/details/windows-mobile-pocket-pc-sdk-repo) | `` | No package | No zip, cab, exe or rar in the item |
| [Windows Moble for Pocket PC 3 XP](https://archive.org/details/preview-32-4791-280) | `` | No package | No zip, cab, exe or rar in the item |
| [Windows Phone 8.1 SDK Tools Lite](https://archive.org/details/WP8.1-SDK-Tools-Lite) | `` | No package | No zip, cab, exe or rar in the item |
| [Windows Phones Apps & Games with Installer Tools](https://archive.org/details/w-10-m-lumia.-7z) | `` | No package | No zip, cab, exe or rar in the item |
| [Windows Pocket PC 2002 (Build 11291)](https://archive.org/details/mobile-2002-beta) | `` | No package | No zip, cab, exe or rar in the item |
| [Windows Pocket PC 2002 build 11291 (x86, VHD)](https://archive.org/details/mobile-2002-beta.-7z) | `` | No package | No zip, cab, exe or rar in the item |
| [Windows Pocket PC 2002 SDK Beta](https://archive.org/details/pocket-pc-2002-sdk-beta) | `` | No package | No zip, cab, exe or rar in the item |
| [Windows Pocket PC 2002 Stuff](https://archive.org/details/WindowsPocketPCStuff-2002) | `ARM_CABs/Tools/PDALight.cab` | Download failed | Download from archive.org failed |
| [WinDriver for Linux](https://archive.org/details/tucows_51688_WinDriver_for_Linux) | `` | No package | No zip, cab, exe or rar in the item |
| [WinFast Navigator - Leadtek GPS](https://archive.org/details/leadtek-gps.toc) | `` | No package | No zip, cab, exe or rar in the item |
| [Wizcode Pocket Hack Master Version 4.14 For The Pocket PC](https://archive.org/details/WizcodePocketHackMasterVersion4.14ForThePocketPC) | `` | No package | No zip, cab, exe or rar in the item |
| [WM 6.1 MPx200 By WinXP ENGLISH](https://archive.org/details/wm6.1mpx200bywinxpenglish) | `` | No package | No zip, cab, exe or rar in the item |
| [WM 6.5 V 21916 HTC Touch 2 Adryan Edition](https://archive.org/details/wm6.5v21916htctouch2adryaneditionfull) | `` | No package | No zip, cab, exe or rar in the item |
| [wm7 SDK (?) of some kind](https://archive.org/details/wm7_sdk) | `` | No package | No zip, cab, exe or rar in the item |
| [X-Ranger](https://archive.org/details/tucows_269883_X-Ranger) | `x-rangersetup.exe` | Download failed | Download from archive.org failed |
| [Zune Software 4.8.2345.0](https://archive.org/details/zune-package_202204) | `` | No package | No zip, cab, exe or rar in the item |
| [Библия - Новый Завет для Windows Mobile смартфонов](https://archive.org/details/bibliya-novy-zavet-windows-mobile) | `` | No package | No zip, cab, exe or rar in the item |
| [Карты "ДубльГИС" за Октябрь 2007](https://archive.org/details/2gis-102007) | `` | No package | No zip, cab, exe or rar in the item |
| [Мобильная коллекция. Смартфоны под управлением Windows Mobile](https://archive.org/details/mediahouse_mobilecollection_winmo) | `` | No package | No zip, cab, exe or rar in the item |
| [Сборник "Лучшие программы для Pocket PC 2005 (Включая ПДД для России 2005)" [RUS] [Disc,Scans]](https://archive.org/details/luchshie-programmi-dlya-pocket-pc-russ) | `` | No package | No zip, cab, exe or rar in the item |
| [Сборник программ "PDA Soft Collection 2" [Disc,Scans]](https://archive.org/details/pda-soft-collection-mhouse) | `` | No package | No zip, cab, exe or rar in the item |
| [Сборник программ "Все для Pocket PC" [RUS] [Disc,Scans]](https://archive.org/details/vse-dlya-pocket-pc-russ) | `` | No package | No zip, cab, exe or rar in the item |
| [Сборник программ "Мир карманных компьютеров 2005. Лучший софт для Pocket PC" [RUS] [Disc,Scans]](https://archive.org/details/mir-karmannih-pc-2005-russ) | `` | No package | No zip, cab, exe or rar in the item |
| [Смартфоны под управлением Windows Mobile](https://archive.org/details/mobi-soft-windows-mobile) | `` | No package | No zip, cab, exe or rar in the item |
| [آپارات/Aparat App for Windows Phone](https://archive.org/details/20250608_20250608_1637) | `` | No package | No zip, cab, exe or rar in the item |
| [小米手机 4刷 Windows 10系统包 10586.1102.3063. Retail](https://archive.org/details/4-windows-10-10586.1102.3063.-retail) | `` | No package | No zip, cab, exe or rar in the item |
| [맵피 Mappy 4.12 완전크랙포함](https://archive.org/details/mappy4.12) | `` | No package | No zip, cab, exe or rar in the item |

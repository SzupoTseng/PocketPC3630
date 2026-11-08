// Part of the PocketPC3630 page, loaded after mfc.js (same global scope, run in order):
// the wall of free games and programs that play with one click.
// ---- Free software wall ----
const CATALOG = /*CATALOG*/ [
  {
    item: 'tucows_164867_Pocket_Quake',
    file: 'PocketQuakeARM.zip',
    title: 'Pocket Quake',
    kind: 'game',
    cpu: 'ARM',
    license: 'GPL＋共享版資料',
    desc: 'Quake 掌上版，附 id Software 的共享版第一章。會一併下載 18 MB 的遊戲資料。',
    shot: 'quake',
    build: 'quake',
  },
  {
    item: 'tucows_200089_Free_Cell',
    file: 'FreeCell_ARM.zip',
    title: 'FreeCell',
    kind: 'game',
    cpu: 'ARM',
    license: 'Tucows',
    desc: '經典新接龍，免費軟體。',
    shot: 'freecell',
  },
  {
    item: 'tucows_182765_Pocket_Slay',
    file: 'PocketSlay.zip',
    title: 'Pocket Slay',
    kind: 'game',
    cpu: 'ARM',
    license: 'Tucows',
    desc: '六角格回合制領土戰略。開局時畫面是黑的：點一下畫面，選難度後按 OK。',
    shot: 'slay',
  },
  {
    item: 'tucows_254025_Pocket_Taipei',
    file: 'ptaipei10.zip',
    title: 'Pocket Taipei',
    kind: 'game',
    cpu: 'ARM',
    license: 'Tucows',
    desc: '麻將牌配對消除。',
    shot: 'taipei',
  },
  {
    item: 'tucows_240055_Blocks_extracted',
    file: 'Blocks/install.PPC_ARM.CAB',
    title: 'Blocks',
    kind: 'game',
    cpu: 'ARM',
    license: 'Tucows',
    desc: '俄羅斯方塊類方塊遊戲。',
    shot: 'blocks',
  },
  {
    item: 'tucows_243676_Hanoi_Tower',
    file: 'CEZebra100b2.exe',
    title: 'CEZebra 黑白棋',
    kind: 'game',
    cpu: 'ARM',
    license: 'Tucows',
    desc: 'CEZebra 黑白棋（Reversi）。',
    shot: 'zebra',
  },
  {
    item: 'tucows_208847_PocketConquest',
    file: 'PocketConquest.zip',
    title: 'PocketConquest',
    kind: 'game',
    cpu: 'ARM',
    license: 'Tucows',
    desc: '世界地圖上的兵棋征服戰略。',
    shot: 'conquest',
  },
  {
    item: 'tucows_309937_GNUChess',
    file: 'GNUChess.zip',
    title: 'GNUChess',
    kind: 'game',
    cpu: 'ARM',
    license: 'Tucows',
    desc: '開源西洋棋引擎 GNU Chess。',
  },
  {
    item: 'tucows_229202_Bubble_Shooter',
    file: 'bubshootce.exe',
    title: 'Bubble Shooter',
    kind: 'game',
    cpu: 'ARM',
    license: 'Tucows',
    desc: '泡泡龍類射擊消除（試用版）。',
    shot: 'tucows_229202_bubble_shooter',
  },
  {
    item: 'tucows_230188_Bubble_Thriller',
    file: 'thrillce.exe',
    title: 'Bubble Thriller',
    kind: 'game',
    cpu: 'ARM',
    license: 'Tucows',
    desc: '泡泡射擊消除（試用版）。',
    shot: 'tucows_230188_bubble_thriller',
  },
  {
    item: 'tucows_309982_Crack_Game',
    file: 'Crack.exe',
    title: 'Crack Game',
    kind: 'game',
    cpu: 'ARM',
    license: 'Tucows',
    desc: '撲克牌接龍遊戲。',
  },
  {
    item: 'tucows_291799_DevilDarts',
    file: 'DevilDartsSetup_demo_arm.exe',
    title: 'DevilDarts',
    kind: 'game',
    cpu: 'ARM',
    license: 'Tucows',
    desc: '射飛鏢（試玩版）。',
    shot: 'tucows_291799_devildarts',
  },
  {
    item: 'tucows_273050_Dragon_Bane_II',
    file: 'DB2SetupARM.exe',
    title: 'Dragon Bane II',
    kind: 'game',
    cpu: 'ARM',
    license: 'Tucows',
    desc: '角色扮演冒險遊戲。',
    shot: 'tucows_273050_dragon_bane_ii',
  },
  {
    item: 'tucows_314055_Elevens_Game',
    file: 'Elevens.exe',
    title: 'Elevens Game',
    kind: 'game',
    cpu: 'ARM',
    license: 'Tucows',
    desc: '撲克牌湊十一接龍。',
  },
  {
    item: 'tucows_250046_Gem_Slider',
    file: 'gslidece.exe',
    title: 'Gem Slider',
    kind: 'game',
    cpu: 'ARM',
    license: 'Tucows',
    desc: '寶石滑動消除（試用版）。',
  },
  {
    item: 'tucows_210362_Line4Line',
    file: 'line4linearm.zip',
    title: 'Line4Line',
    kind: 'game',
    cpu: 'ARM',
    license: 'Tucows',
    desc: '連線益智遊戲。',
    shot: 'tucows_210362_line4line',
  },
  {
    item: 'tucows_222951_Moline',
    file: 'Moline_PocketPC_Trial.exe',
    title: 'Moline',
    kind: 'game',
    cpu: 'ARM',
    license: 'Tucows',
    desc: '益智遊戲（試用版）。',
  },
  {
    item: 'tucows_248733_Nerve',
    file: 'NerveDemoSetup.exe',
    title: 'Nerve',
    kind: 'game',
    cpu: 'ARM',
    license: 'Tucows',
    desc: '動作遊戲（試玩版）。',
    shot: 'tucows_248733_nerve',
  },
  {
    item: 'tucows_250021_Pocket_Pinball',
    file: 'PocketPinballSetupDemo.exe',
    title: 'Pocket Pinball',
    kind: 'game',
    cpu: 'ARM',
    license: 'Tucows',
    desc: '彈珠台（試玩版）。',
    shot: 'tucows_250021_pocket_pinball',
  },
  {
    item: 'tucows_279494_PocketSol',
    file: 'psol20.zip',
    title: 'PocketSol',
    kind: 'game',
    cpu: 'ARM',
    license: 'Tucows',
    desc: '五十多種接龍玩法合集（共享軟體）。',
    shot: 'tucows_279494_pocketsol',
  },
  {
    item: 'tucows_219512_Porta-Pinball',
    file: 'PortaPinSetup300.exe',
    title: 'Porta-Pinball',
    kind: 'game',
    cpu: 'ARM',
    license: 'Tucows',
    desc: '彈珠台。',
    shot: 'tucows_219512_porta-pinball',
  },
  {
    item: 'tucows_312195_Seven_Game',
    file: 'Seven.exe',
    title: 'Seven Game',
    kind: 'game',
    cpu: 'ARM',
    license: 'Tucows',
    desc: '撲克牌遊戲「七」。',
  },
  {
    item: 'tucows_308748_Sorcery_PDA',
    file: 'InstallDXA.exe',
    title: 'Sorcery PDA',
    kind: 'game',
    cpu: 'ARM',
    license: 'Tucows',
    desc: '魔法冒險遊戲。',
  },
  {
    item: 'tucows_32181_Yotz',
    file: 'yotzdemo.exe',
    title: 'Yotz',
    kind: 'game',
    cpu: 'ARM',
    license: 'Tucows',
    desc: '擲骰子遊戲（試玩版）。',
  },
  {
    item: 'tucows_32528_AlarMe',
    file: 'AlarMe_HPC.exe',
    title: 'AlarMe',
    kind: 'tool',
    cpu: 'ARM',
    license: 'Tucows',
    desc: '鬧鐘提醒。',
    shot: 'tucows_32528_alarme',
  },
  {
    item: 'tucows_272210_AW_RoadLingua',
    file: 'rl_ppc.zip',
    title: 'AW RoadLingua',
    kind: 'tool',
    cpu: 'ARM',
    license: 'Tucows',
    desc: '旅遊外語片語。',
    shot: 'tucows_272210_aw_roadlingua',
  },
  {
    item: 'tucows_32347_BasiCE',
    file: 'basice.zip',
    title: 'BasiCE',
    kind: 'tool',
    cpu: 'ARM',
    license: 'Tucows',
    desc: 'BASIC 程式語言直譯器。',
    shot: 'tucows_32347_basice',
  },
  {
    item: 'tucows_212355_Calc98',
    file: 'c9853a.exe',
    title: 'Calc98',
    kind: 'tool',
    cpu: 'ARM',
    license: 'Tucows',
    desc: '工程計算機 Calc98。',
    shot: 'tucows_212355_calc98',
  },
  {
    item: 'tucows_32395_Calendar_Search',
    file: 'CalendarSearchMIPS-PR2.exe',
    title: 'Calendar Search',
    kind: 'tool',
    cpu: 'MIPS',
    license: 'Tucows',
    desc: '行事曆搜尋。',
    shot: 'tucows_32395_calendar_search',
  },
  {
    item: 'tucows_153762_CE-Star_Suite',
    file: 'ver2PkPCARM.zip',
    title: 'CE-Star Suite',
    kind: 'tool',
    cpu: 'ARM',
    license: 'Tucows',
    desc: '文書工具套件。',
    shot: 'tucows_153762_ce-star_suite',
  },
  {
    item: 'tucows_232364_CE5250',
    file: 'CE5250_PocketPC.zip',
    title: 'CE5250',
    kind: 'tool',
    cpu: 'ARM',
    license: 'Tucows',
    desc: 'IBM 5250 終端機模擬。',
    shot: 'tucows_232364_ce5250',
  },
  {
    item: 'tucows_38929_CEDBView',
    file: 'cedbview.zip',
    title: 'CEDBView',
    kind: 'tool',
    cpu: 'ARM',
    license: 'Tucows',
    desc: 'Windows CE 資料庫檢視器。',
  },
  {
    item: 'tucows_32190_CEniffer',
    file: 'CEnifferMIPS.zip.CAB',
    title: 'CEniffer',
    kind: 'tool',
    cpu: 'MIPS',
    license: 'Tucows',
    desc: '網路封包監看。',
    shot: 'tucows_32190_ceniffer',
  },
  {
    item: 'tucows_232362_CETerm',
    file: 'CETerm_PocketPC31.zip',
    title: 'CETerm',
    kind: 'tool',
    cpu: 'ARM',
    license: 'Tucows',
    desc: '終端機模擬程式。',
    shot: 'tucows_232362_ceterm',
  },
  {
    item: 'tucows_232369_CEVT220',
    file: 'CEVT220_PocketPC.zip',
    title: 'CEVT220',
    kind: 'tool',
    cpu: 'ARM',
    license: 'Tucows',
    desc: 'VT220 終端機模擬。',
    shot: 'tucows_232369_cevt220',
  },
  {
    item: 'tucows_32183_Chat',
    file: 'CHAT.zip',
    title: 'Chat',
    kind: 'tool',
    cpu: 'ARM',
    license: 'Tucows',
    desc: '聊天程式。',
    shot: 'tucows_32183_chat',
  },
  {
    item: 'tucows_32399_CodeWallet',
    file: 'cwppcev101.exe',
    title: 'CodeWallet',
    kind: 'tool',
    cpu: 'ARM',
    license: 'Tucows',
    desc: '密碼與個人資料保險箱。',
    shot: 'tucows_32399_codewallet',
  },
  {
    item: 'tucows_358883_CommLog',
    file: 'CommLogCEDesktopSetup.zip',
    title: 'CommLog',
    kind: 'tool',
    cpu: 'ARM',
    license: 'Tucows',
    desc: '通訊紀錄。',
    shot: 'tucows_358883_commlog',
  },
  {
    item: 'tucows_304510_Connex',
    file: 'Connex_PPC_Setup.exe',
    title: 'Connex',
    kind: 'tool',
    cpu: 'ARM',
    license: 'Tucows',
    desc: '網路連線工具。',
    shot: 'tucows_304510_connex',
  },
  {
    item: 'tucows_32301_CoolCalc',
    file: 'coolcalcppc.exe',
    title: 'CoolCalc',
    kind: 'tool',
    cpu: 'ARM',
    license: 'Tucows',
    desc: '計算機。',
    shot: 'tucows_32301_coolcalc',
  },
  {
    item: 'tucows_32532_DeskClock',
    file: 'deskclck.zip',
    title: 'DeskClock',
    kind: 'tool',
    cpu: 'ARM',
    license: 'Tucows',
    desc: '桌面時鐘。',
  },
  {
    item: 'tucows_270214_dotPocket',
    file: 'dotpocket2003.exe',
    title: 'dotPocket',
    kind: 'tool',
    cpu: 'ARM',
    license: 'Tucows',
    desc: '.NET 開發輔助工具。',
    shot: 'tucows_270214_dotpocket',
  },
  {
    item: 'tucows_32354_dsFORTH',
    file: 'ds2_b3_Setup.exe',
    title: 'dsFORTH',
    kind: 'tool',
    cpu: 'ARM',
    license: 'Tucows',
    desc: 'FORTH 程式語言。',
    shot: 'tucows_32354_dsforth',
  },
  {
    item: 'tucows_32496_dTree',
    file: 'dTree.exe',
    title: 'dTree',
    kind: 'tool',
    cpu: 'ARM',
    license: 'Tucows',
    desc: '目錄樹檢視。',
    shot: 'tucows_32496_dtree',
  },
  {
    item: 'tucows_330232_ELFView_-_The_Extremely_Large_File_Viewer',
    file: 'ELFViewARM.CAB',
    title: 'ELFView - The Extremely Large File Viewer',
    kind: 'tool',
    cpu: 'ARM',
    license: 'Tucows',
    desc: '超大檔案檢視器。',
    shot: 'tucows_330232_elfview_-_the_extremely_large_file_viewer',
  },
  {
    item: 'tucows_32432_HPC_Loan_Manager',
    file: 'hpcloan.zip',
    title: 'HPC Loan Manager',
    kind: 'tool',
    cpu: 'ARM',
    license: 'Tucows',
    desc: '貸款試算（Handheld PC 版）。',
    shot: 'tucows_32432_hpc_loan_manager',
  },
  {
    item: 'tucows_32412_HPC_PPC_Stock_Manager',
    file: 'hpcstock.zip',
    title: 'HPC/PPC Stock Manager',
    kind: 'tool',
    cpu: 'ARM',
    license: 'Tucows',
    desc: '股票管理（Handheld PC 版）。',
    shot: 'tucows_32412_hpc_ppc_stock_manager',
  },
  {
    item: 'tucows_32414_IntelliGOLF_Par',
    file: 'igtparce.exe',
    title: 'IntelliGOLF Par',
    kind: 'tool',
    cpu: 'ARM',
    license: 'Tucows',
    desc: '高爾夫計分卡。',
    shot: 'tucows_32414_intelligolf_par',
  },
  {
    item: 'tucows_32505_IrDesk',
    file: 'IrDesk_All.zip',
    title: 'IrDesk',
    kind: 'tool',
    cpu: 'ARM',
    license: 'Tucows',
    desc: '紅外線傳輸工具。',
    shot: 'tucows_32505_irdesk',
  },
  {
    item: 'tucows_150885_Klondike_WAP_Browser',
    file: 'klondikewapsetup.exe',
    title: 'Klondike WAP Browser',
    kind: 'tool',
    cpu: 'ARM',
    license: 'Tucows',
    desc: 'WAP 手機網頁瀏覽器。',
    shot: 'tucows_150885_klondike_wap_browser',
  },
  {
    item: 'tucows_32202_LinkSpy',
    file: 'LinkSpySH3.zip',
    title: 'LinkSpy',
    kind: 'tool',
    cpu: 'SH3',
    license: 'Tucows',
    desc: '連線監看工具。',
    shot: 'tucows_32202_linkspy',
  },
  {
    item: 'tucows_222365_Loan_Manager',
    file: 'pkpcloan_setup.exe',
    title: 'Loan Manager',
    kind: 'tool',
    cpu: 'ARM',
    license: 'Tucows',
    desc: '貸款試算。',
    shot: 'tucows_222365_loan_manager',
  },
  {
    item: 'tucows_328115_Locker',
    file: 'LockerCE_Trial.zip',
    title: 'Locker',
    kind: 'tool',
    cpu: 'ARM',
    license: 'Tucows',
    desc: '檔案加密鎖（試用版）。',
    shot: 'tucows_328115_locker',
  },
  {
    item: 'tucows_216317_MicroCalc_Standard',
    file: 'MicroCalc108.zip',
    title: 'MicroCalc Standard',
    kind: 'tool',
    cpu: 'ARM',
    license: 'Tucows',
    desc: '計算機。',
    shot: 'tucows_216317_microcalc_standard',
  },
  {
    item: 'tucows_32364_miniBasic',
    file: 'minibasic.zip',
    title: 'miniBasic',
    kind: 'tool',
    cpu: 'ARM',
    license: 'Tucows',
    desc: '迷你 BASIC 直譯器。',
    shot: 'tucows_32364_minibasic',
  },
  {
    item: 'tucows_216709_PalmGB',
    file: 'pgbsetup.exe',
    title: 'PalmGB',
    kind: 'tool',
    cpu: 'ARM',
    license: 'Tucows',
    desc: 'Game Boy 模擬器。',
    shot: 'tucows_216709_palmgb',
  },
  {
    item: 'tucows_32646_PalmGB',
    file: 'pgbsetup.exe',
    title: 'PalmGB',
    kind: 'tool',
    cpu: 'ARM',
    license: 'Tucows',
    desc: 'Game Boy 模擬器（舊版）。',
    shot: 'tucows_32646_palmgb',
  },
  {
    item: 'tucows_32648_PilotCE',
    file: 'PilotCE.exe',
    title: 'PilotCE',
    kind: 'tool',
    cpu: 'ARM',
    license: 'Tucows',
    desc: '飛行計畫計算。',
    shot: 'tucows_32648_pilotce',
  },
  {
    item: 'tucows_255472_Pocket_12C',
    file: 'P12CV150ARM.exe',
    title: 'Pocket 12C',
    kind: 'tool',
    cpu: 'ARM',
    license: 'Tucows',
    desc: 'HP 12C 財務計算機模擬。',
    shot: 'tucows_255472_pocket_12c',
  },
  {
    item: 'tucows_294403_Pocket_15C_Scientific_Calculator',
    file: 'Pocket15CV110.exe',
    title: 'Pocket 15C Scientific Calculator',
    kind: 'tool',
    cpu: 'ARM',
    license: 'Tucows',
    desc: 'HP 15C 科學計算機模擬。',
    shot: 'tucows_294403_pocket_15c_scientific_calculator',
  },
  {
    item: 'tucows_252624_Pocket_Tune',
    file: 'PocketTune_2_0_5.zip',
    title: 'Pocket Tune',
    kind: 'tool',
    cpu: 'ARM',
    license: 'Tucows',
    desc: '樂器調音器。',
    shot: 'tucows_252624_pocket_tune',
  },
  {
    item: 'tucows_241852_Pocket_Woman',
    file: 'PocketWoman.zip',
    title: 'Pocket Woman',
    kind: 'tool',
    cpu: 'ARM',
    license: 'Tucows',
    desc: '生理週期行事曆。',
    shot: 'tucows_241852_pocket_woman',
  },
  {
    item: 'tucows_196486_PocketFract',
    file: 'PocketFract.zip',
    title: 'PocketFract',
    kind: 'tool',
    cpu: 'ARM',
    license: 'Tucows',
    desc: '碎形圖產生器。',
    shot: 'tucows_196486_pocketfract',
  },
  {
    item: 'tucows_145763_PocketNav',
    file: 'pocketnav.zip',
    title: 'PocketNav',
    kind: 'tool',
    cpu: 'ARM',
    license: 'Tucows',
    desc: '檔案瀏覽與導覽。',
    shot: 'tucows_145763_pocketnav',
  },
  {
    item: 'tucows_145909_PocketNotepad',
    file: 'pocketnotepad21e.zip',
    title: 'PocketNotepad',
    kind: 'tool',
    cpu: 'ARM',
    license: 'Tucows',
    desc: '記事本。',
    shot: 'tucows_145909_pocketnotepad',
  },
  {
    item: 'tucows_213457_PocketSTM',
    file: 'pocketstmfstrarm.CAB',
    title: 'PocketSTM',
    kind: 'tool',
    cpu: 'ARM',
    license: 'Tucows',
    desc: '時間管理工具。',
    shot: 'tucows_213457_pocketstm',
  },
  {
    item: 'tucows_76856_Primer',
    file: 'PrimerCE.exe',
    title: 'Primer',
    kind: 'tool',
    cpu: 'ARM',
    license: 'Tucows',
    desc: '教學工具。',
    shot: 'tucows_76856_primer',
  },
  {
    item: 'tucows_180072_SEdit',
    file: 'seditmips.zip',
    title: 'SEdit',
    kind: 'tool',
    cpu: 'MIPS',
    license: 'Tucows',
    desc: '文字編輯器。',
    shot: 'tucows_180072_sedit',
  },
  {
    item: 'tucows_216191_SnoopSoft_Dashboard',
    file: 'dashboard.zip',
    title: 'SnoopSoft Dashboard',
    kind: 'tool',
    cpu: 'ARM',
    license: 'Tucows',
    desc: '系統資訊儀表板。',
    shot: 'tucows_216191_snoopsoft_dashboard',
  },
  {
    item: 'tucows_222355_Stock_Manager',
    file: 'pkpcstocks_setup.exe',
    title: 'Stock Manager',
    kind: 'tool',
    cpu: 'ARM',
    license: 'Tucows',
    desc: '股票投資組合管理。',
    shot: 'tucows_222355_stock_manager',
  },
  {
    item: 'tucows_277018_TZO_Dynamic_DNS',
    file: 'tzopocketpc.zip',
    title: 'TZO Dynamic DNS',
    kind: 'tool',
    cpu: 'ARM',
    license: 'Tucows',
    desc: '動態 DNS 用戶端。',
    shot: 'tucows_277018_tzo_dynamic_dns',
  },
  {
    item: 'tucows_245237_UniversalRemoteCE',
    file: 'UniversalRemote_trial.exe',
    title: 'UniversalRemoteCE',
    kind: 'tool',
    cpu: 'ARM',
    license: 'Tucows',
    desc: '紅外線萬用遙控器（試用版）。',
    shot: 'tucows_245237_universalremotece',
  },
  {
    item: 'tucows_289072_VoiceRecorder',
    file: 'VoiceRecorderSetup_TRIAL_tucows.exe',
    title: 'VoiceRecorder',
    kind: 'tool',
    cpu: 'ARM',
    license: 'Tucows',
    desc: '錄音機（試用版）。',
    shot: 'tucows_289072_voicerecorder',
  },
  {
    item: 'tucows_226355_WinEyes',
    file: 'wineyesce.zip',
    title: 'WinEyes',
    kind: 'tool',
    cpu: 'ARM',
    license: 'Tucows',
    desc: '跟著觸控筆轉動的眼睛（經典 xeyes）。',
    shot: 'tucows_226355_wineyes',
  },
  {
    item: 'tucows_32693_XLISP-Plus',
    file: 'xl21gce.zip',
    title: 'XLISP-Plus',
    kind: 'tool',
    cpu: 'ARM',
    license: 'Tucows',
    desc: 'LISP 程式語言直譯器。',
    shot: 'tucows_32693_xlisp-plus',
  },
]; /*END*/
const corsUrl = (item, file) =>
  `https://archive.org/cors/${item}/${file.split('/').map(encodeURIComponent).join('/')}`;

async function download(item, file, label) {
  const response = await fetch(corsUrl(item, file));
  if (!response.ok) throw new Error(t('js.httpError', { status: response.status }));
  const total = Number(response.headers.get('content-length')) || 0;
  const reader = response.body.getReader();
  const parts = [];
  let got = 0;
  for (;;) {
    const { done, value } = await reader.read();
    if (done) break;
    parts.push(value);
    got += value.length;
    say(
      t('js.downloading', {
        label,
        progress: total ? Math.round((got * 100) / total) + '%' : Math.round(got / 1024) + ' KB',
      }),
    );
    busy(
      null,
      t('js.downloading', {
        label,
        progress: total ? Math.round((got * 100) / total) + '%' : Math.round(got / 1024) + ' KB',
      }),
      total ? got / total : null,
    );
  }
  const out = new Uint8Array(got);
  let at = 0;
  for (const p of parts) {
    out.set(p, at);
    at += p.length;
  }
  return out;
}

// A small zip reader and a store-only writer, enough to put Pocket Quake's
// engine and the Quake shareware data into one package in the browser.
async function unzip(bytes) {
  const v = new DataView(bytes.buffer, bytes.byteOffset, bytes.byteLength);
  let eocd = bytes.length - 22;
  while (eocd >= 0 && v.getUint32(eocd, true) !== 0x06054b50) eocd--;
  if (eocd < 0) throw new Error(t('js.notZip'));
  const count = v.getUint16(eocd + 10, true);
  let at = v.getUint32(eocd + 16, true);
  const files = new Map();
  for (let n = 0; n < count; n++) {
    const method = v.getUint16(at + 10, true),
      size = v.getUint32(at + 20, true);
    const nameLen = v.getUint16(at + 28, true),
      extraLen = v.getUint16(at + 30, true),
      commentLen = v.getUint16(at + 32, true);
    const local = v.getUint32(at + 42, true);
    const name = new TextDecoder().decode(bytes.subarray(at + 46, at + 46 + nameLen));
    at += 46 + nameLen + extraLen + commentLen;
    if (name.endsWith('/')) continue;
    const start = local + 30 + v.getUint16(local + 26, true) + v.getUint16(local + 28, true);
    const raw = bytes.subarray(
      start,
      start + v.getUint32(at - 46 - nameLen - extraLen - commentLen + 20, true),
    );
    files.set(name, async () =>
      method === 0
        ? raw.slice(0, size)
        : new Uint8Array(
            await new Response(
              new Blob([raw]).stream().pipeThrough(new DecompressionStream('deflate-raw')),
            ).arrayBuffer(),
          ),
    );
  }
  return files;
}
const CRC = new Uint32Array(256).map((_, n) => {
  let c = n;
  for (let k = 0; k < 8; k++) c = c & 1 ? 0xedb88320 ^ (c >>> 1) : c >>> 1;
  return c >>> 0;
});
const crc32 = (b) => {
  let c = 0xffffffff;
  for (let i = 0; i < b.length; i++) c = CRC[(c ^ b[i]) & 255] ^ (c >>> 8);
  return (c ^ 0xffffffff) >>> 0;
};
function zipStore(entries) {
  const enc = new TextEncoder(),
    chunks = [],
    central = [];
  let offset = 0;
  for (const [name, data] of entries) {
    const n = enc.encode(name),
      crc = crc32(data);
    const h = new DataView(new ArrayBuffer(30));
    h.setUint32(0, 0x04034b50, true);
    h.setUint16(4, 20, true);
    h.setUint32(14, crc, true);
    h.setUint32(18, data.length, true);
    h.setUint32(22, data.length, true);
    h.setUint16(26, n.length, true);
    chunks.push(new Uint8Array(h.buffer), n, data);
    const c = new DataView(new ArrayBuffer(46));
    c.setUint32(0, 0x02014b50, true);
    c.setUint16(4, 20, true);
    c.setUint16(6, 20, true);
    c.setUint32(16, crc, true);
    c.setUint32(20, data.length, true);
    c.setUint32(24, data.length, true);
    c.setUint16(28, n.length, true);
    c.setUint32(42, offset, true);
    central.push(new Uint8Array(c.buffer), n);
    offset += 30 + n.length + data.length;
  }
  const size = central.reduce((a, b) => a + b.length, 0);
  const e = new DataView(new ArrayBuffer(22));
  e.setUint32(0, 0x06054b50, true);
  e.setUint16(8, entries.length, true);
  e.setUint16(10, entries.length, true);
  e.setUint32(12, size, true);
  e.setUint32(16, offset, true);
  return new Blob([...chunks, ...central, new Uint8Array(e.buffer)]);
}

async function buildQuake(engineItem, engineFile) {
  const engine = await unzip(await download(engineItem, engineFile, 'Pocket Quake'));
  const data = await unzip(await download('quakeshareware', 'QUAKE_SW.zip', t('js.quakeData')));
  say(t('js.buildingQuake'));
  const entries = [];
  for (const [name, read] of engine) {
    const base = name.split('/').pop();
    if (/\.(exe|dll)$/i.test(base)) entries.push([`quake/${base}`, await read()]);
    else if (/^autoexec\.cfg$/i.test(base)) entries.push(['quake/id1/autoexec.cfg', await read()]);
  }
  const pak = [...data.keys()].find((k) => /id1\/pak0\.pak$/i.test(k));
  if (!pak) throw new Error(t('js.noPak'));
  entries.push(['quake/id1/pak0.pak', await data.get(pak)()]);
  return new File([zipStore(entries)], 'PocketQuake.zip');
}

// The progress card: shown while a one-click game downloads and starts, so
// the player sees what is happening wherever on the page they clicked.
function busy(title, text, fraction) {
  if (title === false) {
    $('busy').hidden = true;
    return;
  }
  $('busy').hidden = false;
  if (title) $('busy-title').textContent = title;
  if (text != null) $('busy-text').textContent = text;
  const bar = $('busy-bar').parentNode;
  bar.classList.toggle('indeterminate', fraction == null);
  $('busy-bar').style.width = fraction == null ? '' : `${Math.round(fraction * 100)}%`;
}
function findGame(title) {
  const want = title.toLowerCase().replace(/[^a-z0-9]/g, '');
  for (let i = 0; i < M._pocket_game_count(); i++) {
    const name = M.UTF8ToString(M._pocket_game_name(i))
      .toLowerCase()
      .replace(/[^a-z0-9]/g, '');
    if (name && (name === want || name.includes(want) || want.includes(name))) return i;
  }
  return -1;
}
async function getFree(entry, button) {
  button.disabled = true;
  try {
    // Already in the library: no second download, straight into the game.
    const have = findGame(entry.title);
    if (have >= 0) {
      start(have);
      return;
    }
    busy(entry.title, t('js.downloading', { label: entry.title, progress: '' }), null);
    const before = new Set(Array.from({ length: M._pocket_game_count() }, (_, i) => gameId(i)));
    const file =
      entry.build === 'quake'
        ? await buildQuake(entry.item, entry.file)
        : new File([await download(entry.item, entry.file, entry.title)], entry.file.split('/').pop());
    busy(null, t('js.importing', { file: entry.title }), null);
    await importFile(file);
    const count = M._pocket_game_count();
    let target = -1;
    for (let i = 0; i < count; i++)
      if (!before.has(gameId(i))) {
        target = i;
        break;
      }
    if (target < 0) target = findGame(entry.title);
    if (target >= 0) start(target);
  } catch (error) {
    say(t('js.getFailed', { title: entry.title, error: error.message }), true);
    toast(t('js.getFailed', { title: entry.title, error: error.message }));
  } finally {
    busy(false);
    button.disabled = false;
  }
}


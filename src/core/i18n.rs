use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Language {
    Es,
    En,
    De,
    Fr,
    Pt,
    It,
    Ru,
    Zh,
    Ja,
    Ko,
}

impl Default for Language {
    fn default() -> Self {
        Self::Es
    }
}

#[inline(always)]
pub fn tr(
    lang: Language,
    es: &'static str,
    en: &'static str,
    de: &'static str,
    fr: &'static str,
    pt: &'static str,
    it: &'static str,
    ru: &'static str,
    zh: &'static str,
    ja: &'static str,
    ko: &'static str,
) -> &'static str {
    match lang {
        Language::Es => es,
        Language::En => en,
        Language::De => de,
        Language::Fr => fr,
        Language::Pt => pt,
        Language::It => it,
        Language::Ru => ru,
        Language::Zh => zh,
        Language::Ja => ja,
        Language::Ko => ko,
    }
}

#[inline(always)]
pub fn tr2(
    lang: Language,
    es: (&'static str, &'static str),
    en: (&'static str, &'static str),
    de: (&'static str, &'static str),
    fr: (&'static str, &'static str),
    pt: (&'static str, &'static str),
    it: (&'static str, &'static str),
    ru: (&'static str, &'static str),
    zh: (&'static str, &'static str),
    ja: (&'static str, &'static str),
    ko: (&'static str, &'static str),
) -> (&'static str, &'static str) {
    match lang {
        Language::Es => es,
        Language::En => en,
        Language::De => de,
        Language::Fr => fr,
        Language::Pt => pt,
        Language::It => it,
        Language::Ru => ru,
        Language::Zh => zh,
        Language::Ja => ja,
        Language::Ko => ko,
    }
}

#[allow(dead_code)]
impl Language {
    pub const ALL: [Language; 10] = [
        Language::Es,
        Language::En,
        Language::De,
        Language::Fr,
        Language::Pt,
        Language::It,
        Language::Ru,
        Language::Zh,
        Language::Ja,
        Language::Ko,
    ];

    pub fn from_str(s: &str) -> Self {
        match s.trim().to_lowercase().as_str() {
            "es" | "spanish" | "español" | "espanol" => Self::Es,
            "en" | "english" | "inglés" | "ingles" => Self::En,
            "de" | "deutsch" | "german" | "alemán" | "aleman" => Self::De,
            "fr" | "français" | "francais" | "french" | "francés" | "frances" => Self::Fr,
            "pt" | "português" | "portugues" | "portuguese" => Self::Pt,
            "it" | "italiano" | "italian" => Self::It,
            "ru" | "русский" | "russian" | "ruso" => Self::Ru,
            "zh" | "zh-cn" | "chinese" | "chino" | "中文" | "简体中文" => Self::Zh,
            "ja" | "japanese" | "japonés" | "japones" | "日本語" => Self::Ja,
            "ko" | "korean" | "coreano" | "한국어" => Self::Ko,
            _ => Self::Es,
        }
    }

    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Es => "es",
            Self::En => "en",
            Self::De => "de",
            Self::Fr => "fr",
            Self::Pt => "pt",
            Self::It => "it",
            Self::Ru => "ru",
            Self::Zh => "zh",
            Self::Ja => "ja",
            Self::Ko => "ko",
        }
    }

    pub fn code(&self) -> &'static str {
        self.as_str()
    }

    pub fn name(&self) -> &'static str {
        match self {
            Self::Es => "Español",
            Self::En => "English",
            Self::De => "Deutsch",
            Self::Fr => "Français",
            Self::Pt => "Português",
            Self::It => "Italiano",
            Self::Ru => "Русский",
            Self::Zh => "简体中文",
            Self::Ja => "日本語",
            Self::Ko => "한국어",
        }
    }

    pub fn badge(&self) -> &'static str {
        match self {
            Self::Es => "🇪🇸 ES",
            Self::En => "🇬🇧 EN",
            Self::De => "🇩🇪 DE",
            Self::Fr => "🇫🇷 FR",
            Self::Pt => "🇧🇷 PT",
            Self::It => "🇮🇹 IT",
            Self::Ru => "🇷🇺 RU",
            Self::Zh => "🇨🇳 ZH",
            Self::Ja => "🇯🇵 JA",
            Self::Ko => "🇰🇷 KO",
        }
    }

    pub fn next(&self) -> Self {
        match self {
            Self::Es => Self::En,
            Self::En => Self::De,
            Self::De => Self::Fr,
            Self::Fr => Self::Pt,
            Self::Pt => Self::It,
            Self::It => Self::Ru,
            Self::Ru => Self::Zh,
            Self::Zh => Self::Ja,
            Self::Ja => Self::Ko,
            Self::Ko => Self::Es,
        }
    }

    pub fn available_str() -> &'static str {
        "'es' (Español), 'en' (English), 'de' (Deutsch), 'fr' (Français), 'pt' (Português), 'it' (Italiano), 'ru' (Русский), 'zh' (简体中文), 'ja' (日本語), 'ko' (한국어)"
    }

    // --- NAVIGATION TABS ---
    pub fn tab_fans(&self) -> &'static str {
        tr(*self,
            "🌀 VENTILADORES (1)",
            "🌀 FAN CONTROL (1)",
            "🌀 LÜFTERSTEUERUNG (1)",
            "🌀 VENTILATION (1)",
            "🌀 VENTOINHAS (1)",
            "🌀 VENTILATORI (1)",
            "🌀 ВЕНТИЛЯТОРЫ (1)",
            "🌀 风扇控制 (1)",
            "🌀 ファン制御 (1)",
            "🌀 팬 제어 (1)",
        )
    }

    pub fn tab_monitoring(&self) -> &'static str {
        tr(*self,
            "📊 MONITOREO (2)",
            "📊 MONITORING (2)",
            "📊 ÜBERWACHUNG (2)",
            "📊 SURVEILLANCE (2)",
            "📊 MONITORAMENTO (2)",
            "📊 MONITORAGGIO (2)",
            "📊 МОНИТОРИНГ (2)",
            "📊 硬件监控 (2)",
            "📊 ハードウェア監視 (2)",
            "📊 모니터링 (2)",
        )
    }

    pub fn tab_power(&self) -> &'static str {
        tr(*self,
            "⚡ ESCENARIOS (3)",
            "⚡ POWER MODES (3)",
            "⚡ LEISTUNGSMODI (3)",
            "⚡ MODES D'ÉNERGIE (3)",
            "⚡ MODOS DE ENERGIA (3)",
            "⚡ MODALITÀ ENERGIA (3)",
            "⚡ РЕЖИМЫ ПИТАНИЯ (3)",
            "⚡ 性能模式 (3)",
            "⚡ 動作モード (3)",
            "⚡ 전원 모드 (3)",
        )
    }

    pub fn tab_rgb(&self) -> &'static str {
        tr(*self,
            "🌈 ILUMINACIÓN (4)",
            "🌈 LIGHTING (4)",
            "🌈 BELEUCHTUNG (4)",
            "🌈 ÉCLAIRAGE (4)",
            "🌈 ILUMINAÇÃO (4)",
            "🌈 ILLUMINAZIONE (4)",
            "🌈 ПОДСВЕТКА (4)",
            "🌈 键盘背光 (4)",
            "🌈 キーボード照明 (4)",
            "🌈 키보드 조명 (4)",
        )
    }

    pub fn tab_settings(&self) -> &'static str {
        tr(*self,
            "⚙ AJUSTES (5)",
            "⚙ SETTINGS (5)",
            "⚙ EINSTELLUNGEN (5)",
            "⚙ PARAMÈTRES (5)",
            "⚙ CONFIGURAÇÕES (5)",
            "⚙ IMPOSTAZIONI (5)",
            "⚙ НАСТРОЙКИ (5)",
            "⚙ 系统设置 (5)",
            "⚙ 環境設定 (5)",
            "⚙ 시스템 설정 (5)",
        )
    }

    // --- FOOTER & QUICK BAR ---
    pub fn footer_ac(&self) -> &'static str {
        tr(*self,
            "⚡ RED AC (135W)",
            "⚡ AC MAINS (135W)",
            "⚡ NETZSTROM (135W)",
            "⚡ SECTEUR AC (135W)",
            "⚡ REDE AC (135W)",
            "⚡ RETE AC (135W)",
            "⚡ СЕТЬ 220V (135W)",
            "⚡ 交流供电 (135W)",
            "⚡ AC電源 (135W)",
            "⚡ AC 전원 (135W)",
        )
    }

    pub fn footer_battery(&self, pct: u8) -> String {
        let fmt = tr(*self,
            "🔋 BATERÍA ({}%)",
            "🔋 BATTERY ({}%)",
            "🔋 AKKU ({}%)",
            "🔋 BATTERIE ({}%)",
            "🔋 BATERIA ({}%)",
            "🔋 BATTERIA ({}%)",
            "🔋 БАТАРЕЯ ({}%)",
            "🔋 电池 ({}%)",
            "🔋 バッテリー ({}%)",
            "🔋 배터리 ({}%)",
        );
        fmt.replace("{}", &pct.to_string())
    }

    pub fn footer_fan(&self, mode: &str) -> String {
        let fmt = tr(*self,
            "🌀 MODO: {}",
            "🌀 FAN: {}",
            "🌀 LÜFTER: {}",
            "🌀 VENTILATION: {}",
            "🌀 VENTOINHA: {}",
            "🌀 VENTOLE: {}",
            "🌀 КУЛЕР: {}",
            "🌀 风扇模式: {}",
            "🌀 ファン: {}",
            "🌀 팬 모드: {}",
        );
        fmt.replace("{}", &mode.to_uppercase())
    }

    pub fn footer_profile(&self, profile: &str) -> String {
        let fmt = tr(*self,
            "⚡ PERFIL: {}",
            "⚡ PROFILE: {}",
            "⚡ PROFIL: {}",
            "⚡ PROFIL: {}",
            "⚡ PERFIL: {}",
            "⚡ PROFILO: {}",
            "⚡ ПРОФИЛЬ: {}",
            "⚡ 模式: {}",
            "⚡ プロファイル: {}",
            "⚡ 프로필: {}",
        );
        fmt.replace("{}", &profile.to_uppercase())
    }

    pub fn footer_dissipation(&self, watts: f32) -> String {
        let fmt = tr(*self,
            "DISIPACIÓN: {:.1} W",
            "DISSIPATION: {:.1} W",
            "VERLUSTLEISTUNG: {:.1} W",
            "DISSIPATION: {:.1} W",
            "DISSIPAÇÃO: {:.1} W",
            "DISSIPAZIONE: {:.1} W",
            "ТЕПЛОВЫДЕЛЕНИЕ: {:.1} W",
            "总功耗散热: {:.1} W",
            "消費電力放熱: {:.1} W",
            "소비 전력 방열: {:.1} W",
        );
        format!("{}", fmt.replace("{:.1}", &format!("{:.1}", watts)))
    }

    pub fn footer_shortcuts(&self) -> &'static str {
        tr(*self,
            "[1-5] Pestañas  [T] Tema  [L] Idioma  [A/M/C] Modo  [Ctrl+Q] Salir",
            "[1-5] Tabs  [T] Theme  [L] Language  [A/M/C] Mode  [Ctrl+Q] Exit",
            "[1-5] Tabs  [T] Thema  [L] Sprache  [A/M/C] Modus  [Strg+Q] Beenden",
            "[1-5] Onglets  [T] Thème  [L] Langue  [A/M/C] Mode  [Ctrl+Q] Quitter",
            "[1-5] Abas  [T] Tema  [L] Idioma  [A/M/C] Modo  [Ctrl+Q] Sair",
            "[1-5] Schede  [T] Tema  [L] Lingua  [A/M/C] Modalità  [Ctrl+Q] Esci",
            "[1-5] Вкладки  [T] Тема  [L] Язык  [A/M/C] Режим  [Ctrl+Q] Выход",
            "[1-5] 切换选项卡  [T] 视觉主题  [L] 界面语言  [A/M/C] 模式  [Ctrl+Q] 退出",
            "[1-5] タブ切替  [T] テーマ  [L] 言語  [A/M/C] モード  [Ctrl+Q] 終了",
            "[1-5] 탭 전환  [T] 테마  [L] 언어  [A/M/C] 모드  [Ctrl+Q] 종료",
        )
    }

    pub fn theme_tooltip(&self) -> &'static str {
        tr(*self,
            "Clic o presiona [T] para alternar entre paletas ergonómicas",
            "Click or press [T] to cycle visual ergonomics themes",
            "Klicken oder [T] drücken, um Farbthemen zu wechseln",
            "Cliquez ou appuyez sur [T] pour changer de thème visuel",
            "Clique ou pressione [T] para alternar temas visuais",
            "Fai clic o premi [T] per scorrere i temi visivi",
            "Нажмите [T] для переключения цветовой темы",
            "点击或按 [T] 键切换护眼视觉色彩主题",
            "クリックまたは [T] キーで配色テーマを切り替え",
            "클릭하거나 [T] 키를 눌러 비주얼 테마 전환",
        )
    }

    pub fn lang_tooltip(&self) -> &'static str {
        tr(*self,
            "Clic o presiona [L] para cambiar de idioma",
            "Click or press [L] to change language",
            "Klicken oder [L] drücken, um Sprache zu ändern",
            "Cliquez ou appuyez sur [L] pour changer de langue",
            "Clique ou pressione [L] para mudar o idioma",
            "Fai clic o premi [L] per cambiare lingua",
            "Нажмите [L] для смены языка интерфейса",
            "点击或按 [L] 键切换下一个界面语言",
            "クリックまたは [L] キーで言語を切り替え",
            "클릭하거나 [L] 키를 눌러 언어 전환",
        )
    }

    // --- FANS TAB ---
    pub fn fan_title(&self) -> &'static str {
        tr(*self,
            "Control de Velocidad de Ventiladores",
            "Fan Speed Control",
            "Lüftergeschwindigkeitssteuerung",
            "Contrôle de vitesse des ventilateurs",
            "Controle de Velocidade das Ventoinhas",
            "Controllo velocità ventole",
            "Управление скоростью вентиляторов",
            "散热风扇转速与风道控制",
            "冷却ファン速度制御",
            "냉각 팬 속도 제어",
        )
    }

    pub fn fan_mode_badge(&self, mode: &str) -> (&'static str, &'static str) {
        match mode {
            "auto" => tr2(*self,
                ("• LAZO CERRADO BIOS", "Lazo cerrado autónomo del EC Compal. Ajuste dinámico según curvas térmicas."),
                ("• BIOS CLOSED LOOP", "Compal EC autonomous closed loop. Fans dynamically ramp according to thermal curves."),
                ("• BIOS-REGELKREIS", "Autonomer Compal EC-Regelkreis. Dynamische Anpassung nach Kurven."),
                ("• BOUCLE FERMÉE BIOS", "Boucle autonome Compal EC. Ajustement dynamique selon courbes thermiques."),
                ("• LAÇO FECHADO BIOS", "Laço fechado autônomo do EC Compal. Ajuste dinâmico por curvas térmicas."),
                ("• ANELLO CHIUSO BIOS", "Anello chiuso autonomo Compal EC. Regolazione dinamica per curve termiche."),
                ("• АВТОМАТИКА BIOS", "Автономный контур Compal EC. Динамическая регулировка по термокривым."),
                ("• BIOS 自主闭环", "Compal EC 固件自主控温闭环，依据出厂热工阻抗曲线自适应调节。"),
                ("• BIOS 自律制御", "Compal EC 自律クローズドループ。温度プロファイルに基づき自動調整。"),
                ("• BIOS 자율 제어", "Compal EC 독립형 폐루프 제어. 열 커브에 따라 동적 자동 조절."),
            ),
            "max" | "turbo" => tr2(*self,
                ("• SOBREACELERACIÓN SMM TURBO", "Sobreaceleración activa. Registros del EC forzados a 0xFF (12V en turbinas)."),
                ("• TURBO SMM OVERDRIVE", "Overdrive active. Compal EC registers locked to 0xFF (Max 12V fan rail voltage)."),
                ("• TURBO-SMM-OVERDRIVE", "Overdrive aktiv. EC-Register auf 0xFF fixiert (Maximale 12V-Lüfterspannung)."),
                ("• OVERDRIVE TURBO SMM", "Overdrive actif. Registres EC forcés à 0xFF (Tension maximale 12V)."),
                ("• SOBREACELERAÇÃO TURBO SMM", "Overdrive ativo. Registros EC forçados para 0xFF (Tensão máxima 12V)."),
                ("• OVERDRIVE TURBO SMM", "Overdrive attivo. Registri EC forzati a 0xFF (Tensione massima 12V)."),
                ("• ТУРБО-ОВЕРДРАЙВ SMM", "Турбо-режим активен. Регистры EC зафиксированы на 0xFF (12V максимум)."),
                ("• SMM 极速狂暴超频", "SMM 硬件狂暴模式生效，EC 寄存器强制置位 0xFF，全速 12V 满血运转。"),
                ("• SMM ターボ稼働", "ターボオーバードライブ稼働。ECレジスタを0xFF固定（最大12V駆動）。"),
                ("• SMM 터보 오버드라이브", "오버드라이브 가동 중. EC 레지스터 0xFF 고정 (최대 12V 풀 출력)."),
            ),
            _ => tr2(*self,
                ("• OBJETIVO MANUAL PWM", "Anulación manual de ciclo PWM. Lazo cerrado desplazado al objetivo fijado."),
                ("• MANUAL PWM TARGET", "Manual PWM duty cycle override. Autonomous closed loop slipped to target duty."),
                ("• MANUELLER PWM-WERT", "Manuelle PWM-Vorgabe. Fester Lüfterzielwert aktiv."),
                ("• CIBLE PWM MANUELLE", "Dérogation manuelle du rapport cyclique PWM."),
                ("• ALVO MANUAL PWM", "Substituição manual do ciclo de trabalho PWM."),
                ("• TARGET PWM MANUALE", "Controllo manuale del duty cycle PWM."),
                ("• РУЧНОЙ РЕЖИМ PWM", "Ручная фиксация коэффициента заполнения ШИМ."),
                ("• 手动 PWM 占空比", "用户自定义固定占空比调速，覆盖 BIOS 默认曲线。"),
                ("• 手動 PWM 設定", "PWM デューティ比の手動固定。目標値へ強制設定。"),
                ("• 수동 PWM 목표치", "수동 PWM 듀티 사이클 설정. 지정된 목표 속도로 고정."),
            ),
        }
    }

    pub fn coolboost_text(&self, active: bool) -> &'static str {
        if active {
            tr(*self,
                "❄ CoolBoost™ ACTIVO",
                "❄ CoolBoost™ ON",
                "❄ CoolBoost™ AKTIV",
                "❄ CoolBoost™ ACTIF",
                "❄ CoolBoost™ ATIVO",
                "❄ CoolBoost™ ATTIVO",
                "❄ CoolBoost™ ВКЛ",
                "❄ CoolBoost™ 已开启",
                "❄ CoolBoost™ オン",
                "❄ CoolBoost™ 활성",
            )
        } else {
            tr(*self,
                "❄ CoolBoost™ OFF",
                "❄ CoolBoost™ OFF",
                "❄ CoolBoost™ AUS",
                "❄ CoolBoost™ INACTIF",
                "❄ CoolBoost™ DESAT",
                "❄ CoolBoost™ DISATT",
                "❄ CoolBoost™ ВЫКЛ",
                "❄ CoolBoost™ 已关闭",
                "❄ CoolBoost™ オフ",
                "❄ CoolBoost™ 끔",
            )
        }
    }

    pub fn coolboost_tooltip(&self) -> &'static str {
        tr(*self,
            "Compal EC CoolBoost: eleva la curva base en +400-500 RPM para reducir la histéresis térmica. Clic para alternar.",
            "Compal EC CoolBoost offset: elevates dynamic fan curve baseline +400-500 RPM for lower thermal hysteresis. Click to toggle.",
            "Compal EC CoolBoost: Hebt die Grundlüfterkurve um +400-500 U/min an, um thermische Spitzen abzufedern.",
            "Compal EC CoolBoost : augmente la courbe de base de +400-500 tr/min pour réduire l'hystérésis thermique.",
            "Compal EC CoolBoost: eleva a curva base em +400-500 RPM para amortecer picos térmicos.",
            "Compal EC CoolBoost: aumenta la curva di base di +400-500 RPM per ridurre l'isteresi termica.",
            "Compal EC CoolBoost: поднимает базовую кривую на +400-500 об/мин для быстрого сброса тепла.",
            "Compal EC CoolBoost 酷冷技术：将风扇基准转速动态拔高 400-500 RPM，降低突发高负载热滞后。",
            "Compal EC CoolBoost: ファンの基本回転数を +400-500 RPM 引き上げ、熱ヒステリシスを抑制します。",
            "Compal EC CoolBoost: 팬 베이스라인을 +400-500 RPM 상승시켜 순간적인 발열 지연을 방지합니다.",
        )
    }

    pub fn sync_fans_text(&self, sync: bool) -> &'static str {
        if sync {
            tr(*self,
                "🔗 Turbinas Vinculadas",
                "🔗 Fans Linked",
                "🔗 Lüfter Gekoppelt",
                "🔗 Ventilateurs Liés",
                "🔗 Ventoinhas Vinculadas",
                "🔗 Ventole Collegate",
                "🔗 Кулеры Связаны",
                "🔗 风扇协同联动",
                "🔗 ファン連動中",
                "🔗 팬 동기화 연동",
            )
        } else {
            tr(*self,
                "⚡ Independientes",
                "⚡ Independent",
                "⚡ Unabhängig",
                "⚡ Indépendants",
                "⚡ Independentes",
                "⚡ Indipendenti",
                "⚡ Независимо",
                "⚡ 独立分控",
                "⚡ 個別制御",
                "⚡ 개별 독립 제어",
            )
        }
    }

    pub fn sync_fans_tooltip(&self, sync: bool) -> &'static str {
        if sync {
            tr(*self,
                "Ventiladores VINCULADOS: CPU y GPU se mueven en sincronía. Clic para desacoplar.",
                "Fans are LINKED: CPU and GPU target speeds move synchronously. Click to decouple.",
                "Lüfter GEKOPPELT: CPU- und GPU-Drehzahlen synchron. Klicken zum Entkoppeln.",
                "Ventilateurs LIÉS : vitesses CPU et GPU synchrones. Cliquez pour dissocier.",
                "Ventoinhas VINCULADAS: CPU e GPU movem-se sincronizadas. Clique para desacoplar.",
                "Ventole COLLEGATE: velocità CPU e GPU sincronizzate. Clic per scollegare.",
                "Кулеры СВЯЗАНЫ: скорости CPU и GPU изменяются синхронно. Нажмите для разделения.",
                "双涡轮风扇联动：CPU 与 GPU 设定值保持同步联动。点击可解除联动分控。",
                "ファン連動：CPUとGPUの目標回転数が同期します。クリックで連動解除。",
                "팬 연동 상태: CPU 및 GPU 속도가 함께 조절됩니다. 클릭하여 분리 제어.",
            )
        } else {
            tr(*self,
                "Ventiladores INDEPENDIENTES: CPU y GPU se ajustan por separado. Clic para vincular.",
                "Fans are INDEPENDENT: CPU and GPU target speeds are adjusted separately. Click to link.",
                "Lüfter UNABHÄNGIG: CPU und GPU separat gesteuert. Klicken zum Koppeln.",
                "Ventilateurs INDÉPENDANTS : ajustement séparé. Cliquez pour synchroniser.",
                "Ventoinhas INDEPENDENTES: CPU e GPU ajustadas separadamente. Clique para vincular.",
                "Ventole INDIPENDENTI: regolate separatamente. Clic per collegare.",
                "Кулеры РАЗДЕЛЬНЫ: CPU и GPU настраиваются отдельно. Нажмите для синхронизации.",
                "风扇独立分控：CPU 和 GPU 可单独设定目标占空比。点击开启协同联动。",
                "ファン個別制御：CPUとGPUを個別に設定します。クリックで同期連动。",
                "팬 독립 제어: CPU와 GPU를 각자 조절합니다. 클릭하여 동기화 연동.",
            )
        }
    }

    pub fn btn_auto_label(&self, compact: bool) -> &'static str {
        if compact {
            "AUTO (A)"
        } else {
            tr(*self,
                "AUTO (A)\nCurva BIOS",
                "AUTO (A)\nBIOS Closed Loop",
                "AUTO (A)\nBIOS-Kurve",
                "AUTO (A)\nCourbe BIOS",
                "AUTO (A)\nCurva BIOS",
                "AUTO (A)\nCurva BIOS",
                "АВТО (A)\nКривая BIOS",
                "自动 (A)\nBIOS闭环曲线",
                "自動 (A)\nBIOS自律制御",
                "자동 (A)\nBIOS 자동제어",
            )
        }
    }

    pub fn btn_max_label(&self, compact: bool) -> &'static str {
        if compact {
            tr(*self, "MÁX (M)", "MAX (M)", "MAX (M)", "MAX (M)", "MÁX (M)", "MAX (M)", "МАКС (M)", "最大 (M)", "最大 (M)", "최대 (M)")
        } else {
            tr(*self,
                "MÁXIMO (M)\n100% Turbo",
                "MAX (M)\n100% Turbo",
                "MAXIMAL (M)\n100% Turbo",
                "MAXIMUM (M)\n100% Turbo",
                "MÁXIMO (M)\n100% Turbo",
                "MASSIMO (M)\n100% Turbo",
                "МАКСИМУМ (M)\n100% Турбо",
                "最大 (M)\n100% 极速狂暴",
                "最大 (M)\n100% ターボ",
                "최대 (M)\n100% 터보",
            )
        }
    }

    pub fn btn_custom_label(&self, compact: bool) -> &'static str {
        if compact {
            tr(*self, "MANUAL (C)", "CUSTOM (C)", "MANUELL (C)", "MANUEL (C)", "MANUAL (C)", "MANUALE (C)", "РУЧНОЙ (C)", "自定义 (C)", "カスタム (C)", "수동 (C)")
        } else {
            tr(*self,
                "MANUAL (C)\nObjetivo Fijo",
                "CUSTOM (C)\nManual Target",
                "BENUTZER (C)\nFeste Vorgabe",
                "MANUEL (C)\nCible Fixe",
                "MANUAL (C)\nAlvo Fixo",
                "MANUALE (C)\nTarget Fisso",
                "РУЧНОЙ (C)\nФикс. уровень",
                "自定义 (C)\n手动定速",
                "カスタム (C)\n固定目標",
                "수동 (C)\n고정 목표",
            )
        }
    }

    pub fn presets_title(&self) -> &'static str {
        tr(*self,
            "Ajustes rápidos:",
            "Presets:",
            "Schnellwahl:",
            "Préréglages :",
            "Ajustes rápidos:",
            "Preimpostazioni:",
            "Быстрые пресеты:",
            "预设挡位:",
            "クイックプリセット:",
            "빠른 프리셋:",
        )
    }

    pub fn fan_presets(&self) -> [(&'static str, u8, &'static str); 4] {
        match self {
            Self::Es => [
                ("20% Silencioso", 20, "20% Objetivo (~2100 RPM) • Mínima huella acústica"),
                ("45% Equilibrado", 45, "45% Objetivo (~3200 RPM) • Navegación y programación"),
                ("75% Alto Flujo", 75, "75% Objetivo (~4500 RPM) • Compilación y gaming sostenido"),
                ("100% Máximo", 100, "100% Objetivo (RPM máx.) • Prevención de estrangulamiento térmico"),
            ],
            Self::En => [
                ("20% Quiet", 20, "20% Target (~2100 RPM) • Low acoustic footprint"),
                ("45% Balanced", 45, "45% Target (~3200 RPM) • Daily browsing & dev"),
                ("75% High Flow", 75, "75% Target (~4500 RPM) • Sustained compile & gaming"),
                ("100% Max", 100, "100% Target (Max RPM) • Thermal throttling prevention"),
            ],
            Self::De => [
                ("20% Leise", 20, "20% Ziel (~2100 U/min) • Kaum hörbar für Office"),
                ("45% Ausgewogen", 45, "45% Ziel (~3200 U/min) • Browsen & Entwicklung"),
                ("75% Hoher Durchsatz", 75, "75% Ziel (~4500 U/min) • Kompilieren & Gaming"),
                ("100% Maximum", 100, "100% Ziel (Volle Drehzahl) • Throttling-Schutz"),
            ],
            Self::Fr => [
                ("20% Silencieux", 20, "Cible 20% (~2100 tr/min) • Empreinte sonore minime"),
                ("45% Équilibré", 45, "Cible 45% (~3200 tr/min) • Navigation & dev"),
                ("75% Haut Débit", 75, "Cible 75% (~4500 tr/min) • Compilation & gaming"),
                ("100% Maximum", 100, "Cible 100% (Vitesse max) • Prévention thermique"),
            ],
            Self::Pt => [
                ("20% Silencioso", 20, "Alvo 20% (~2100 RPM) • Pegada acústica mínima"),
                ("45% Equilibrado", 45, "Alvo 45% (~3200 RPM) • Navegação e desenvolvimento"),
                ("75% Alto Fluxo", 75, "Alvo 75% (~4500 RPM) • Compilação e jogos pesados"),
                ("100% Máximo", 100, "Alvo 100% (RPM máx.) • Prevenção de estrangulamento"),
            ],
            Self::It => [
                ("20% Silenzioso", 20, "Target 20% (~2100 RPM) • Minima rumorosità"),
                ("45% Bilanciato", 45, "Target 45% (~3200 RPM) • Navigazione e dev"),
                ("75% Alto Flusso", 75, "Target 75% (~4500 RPM) • Compilazione e gaming"),
                ("100% Massimo", 100, "Target 100% (RPM max) • Prevenzione throttling"),
            ],
            Self::Ru => [
                ("20% Тихий", 20, "20% (~2100 об/мин) • Минимальный уровень шума"),
                ("45% Базовый", 45, "45% (~3200 об/мин) • Работа в браузере и кодинг"),
                ("75% Высокий", 75, "75% (~4500 об/мин) • Компиляция и игры"),
                ("100% Максимум", 100, "100% (Макс. об/мин) • Защита от троттлинга"),
            ],
            Self::Zh => [
                ("20% 静音节能", 20, "20% 目标 (~2100 RPM) • 极低风噪，轻度办公"),
                ("45% 均衡日常", 45, "45% 目标 (~3200 RPM) • 日常浏览与工程开发"),
                ("75% 高风量输出", 75, "75% 目标 (~4500 RPM) • 代码重度编译与竞技游戏"),
                ("100% 极速狂暴", 100, "100% 目标 (最高转速) • 压制热节流降频"),
            ],
            Self::Ja => [
                ("20% 静音", 20, "20% 目標 (~2100 RPM) • 最小動作音・事務用"),
                ("45% バランス", 45, "45% 目標 (~3200 RPM) • 通常作業＆プログラミング"),
                ("75% 高風量", 75, "75% 目標 (~4500 RPM) • 高負荷処理＆ゲーミング"),
                ("100% フルパワー", 100, "100% 目標 (最大回転) • サーマルスロットリング防止"),
            ],
            Self::Ko => [
                ("20% 저소음", 20, "20% 목표 (~2100 RPM) • 최저 소음 및 사무용"),
                ("45% 밸런스", 45, "45% 목표 (~3200 RPM) • 웹 서핑 및 개발 환경"),
                ("75% 고유량", 75, "75% 목표 (~4500 RPM) • 컴파일 및 게이밍 모드"),
                ("100% 풀 파워", 100, "100% 목표 (최대 RPM) • 스로틀링 방지 극대화"),
            ],
        }
    }

    pub fn cpu_fan_name(&self) -> &'static str {
        tr(*self,
            "VENTILADOR CPU", "CPU FAN", "CPU-LÜFTER", "VENTILATEUR CPU", "VENTOINHA CPU",
            "VENTOLA CPU", "КУЛЕР CPU", "CPU 风扇", "CPU ファン", "CPU 팬",
        )
    }

    pub fn gpu_fan_name(&self) -> &'static str {
        tr(*self,
            "VENTILADOR GPU", "GPU FAN", "GPU-LÜFTER", "VENTILATEUR GPU", "VENTOINHA GPU",
            "VENTOLA GPU", "КУЛЕР GPU", "GPU 风扇", "GPU ファン", "GPU 팬",
        )
    }

    pub fn delta_trend_str(&self, delta: i32) -> &'static str {
        if delta > 35 {
            tr(*self,
                "Acelerando ▲", "Spinning Up ▲", "Beschleunigt ▲", "Accélération ▲", "Acelerando ▲",
                "Accelerando ▲", "Разгон ▲", "加速中 ▲", "加速中 ▲", "가속 중 ▲",
            )
        } else if delta < -35 {
            tr(*self,
                "Desacelerando ▼", "Slowing Down ▼", "Verlangsamt ▼", "Ralentissement ▼", "Desacelerando ▼",
                "Rallentando ▼", "Замедление ▼", "减速中 ▼", "減速中 ▼", "감속 중 ▼",
            )
        } else {
            tr(*self,
                "Estable •", "Steady •", "Stabil •", "Stable •", "Estável •",
                "Stabile •", "Стабильно •", "转速稳定 •", "安定 •", "안정적 •",
            )
        }
    }

    pub fn dial_telemetry_hover(&self, title: &str, temp: f32, headroom: f32, tjmax: f32) -> String {
        let fmt = tr(*self,
            "{} Telemetría:\n• Temp actual: {:.1}°C\n• Margen térmico: +{:.1}°C antes de Throttling (Tope: {:.0}°C)",
            "{} Telemetry:\n• Current temp: {:.1}°C\n• Thermal headroom: +{:.1}°C before Throttling (Limit: {:.0}°C)",
            "{} Telemetrie:\n• Aktuelle Temp: {:.1}°C\n• Thermischer Spielraum: +{:.1}°C bis Throttling (Limit: {:.0}°C)",
            "{} Télémétrie :\n• Temp actuelle : {:.1}°C\n• Marge thermique : +{:.1}°C avant throttling (Limite : {:.0}°C)",
            "{} Telemetria:\n• Temp atual: {:.1}°C\n• Margem térmica: +{:.1}°C antes de Throttling (Limite: {:.0}°C)",
            "{} Telemetria:\n• Temp attuale: {:.1}°C\n• Margine termico: +{:.1}°C prima del throttling (Limite: {:.0}°C)",
            "{} Телеметрия:\n• Тек. темп: {:.1}°C\n• Тепловой запас: +{:.1}°C до троттлинга (Предел: {:.0}°C)",
            "{} 实时监控:\n• 当前核心温度: {:.1}°C\n• 热安全余量: +{:.1}°C 距临界降频 (TjMax: {:.0}°C)",
            "{} テレメトリ:\n• 現在温度: {:.1}°C\n• サーマルヘッドルーム: +{:.1}°C (限界: {:.0}°C)",
            "{} 텔레메트리:\n• 현재 온도: {:.1}°C\n• 여유 열 마진: +{:.1}°C 스로틀링 기준 (한계: {:.0}°C)",
        );
        format!("{}", fmt.replace("{}", title).replace("{:.1}", &format!("{:.1}", temp)).replace("{:.1}", &format!("{:.1}", headroom)).replace("{:.0}", &format!("{:.0}", tjmax)))
    }

    pub fn minus_tooltip(&self) -> &'static str {
        tr(*self,
            "Reducir -5%", "Decrease -5%", "Verringern -5%", "Diminuer -5%", "Reduzir -5%",
            "Riduci -5%", "Уменьшить -5%", "降低 -5%", "5% 減少", "5% 감소",
        )
    }

    pub fn plus_tooltip(&self) -> &'static str {
        tr(*self,
            "Aumentar +5%", "Increase +5%", "Erhöhen +5%", "Augmenter +5%", "Aumentar +5%",
            "Aumenta +5%", "Увеличить +5%", "提高 +5%", "5% 増加", "5% 증가",
        )
    }

    pub fn turbo_smm_badge(&self) -> (&'static str, &'static str) {
        tr2(*self,
            ("⚡ Overdrive Turbo (100% SMM)", "SMM Turbo activo forzando raíles a máxima potencia de disipación."),
            ("⚡ Turbo Overdrive (100% SMM)", "SMM Turbo active forcing fan rails to maximum cooling power."),
            ("⚡ Turbo-Overdrive (100% SMM)", "SMM-Turbo aktiv. Lüfterschienen auf maximale Kühlleistung erzwungen."),
            ("⚡ Overdrive Turbo (100% SMM)", "Turbo SMM actif forçant la puissance de refroidissement maximale."),
            ("⚡ Overdrive Turbo (100% SMM)", "Turbo SMM ativo forçando trilhos à máxima potência de dissipação."),
            ("⚡ Overdrive Turbo (100% SMM)", "SMM Turbo attivo per la massima potenza di dissipazione termica."),
            ("⚡ Турбо-овердрайв (100% SMM)", "SMM Turbo активен: максимальная мощность охлаждения."),
            ("⚡ 狂暴 Turbo 超频 (100% SMM)", "SMM 硬件直控满血输出，风扇供电轨全开至极致散热状态。"),
            ("⚡ ターボオーバードライブ (100% SMM)", "SMM ターボ有効。ファンを最大放熱能力で強制駆動。"),
            ("⚡ 터보 오버드라이브 (100% SMM)", "SMM 터보 활성화: 팬 레일 최대 방열 출력 고정."),
        )
    }

    pub fn bios_curve_badge(&self) -> (&'static str, &'static str) {
        tr2(*self,
            ("🔒 Curva Térmica BIOS Autónoma", "Compal EC gestiona dinámicamente el ciclo PWM según la curva térmica."),
            ("🔒 Autonomous BIOS Thermal Curve", "Compal EC dynamically manages PWM duty based on thermal curve."),
            ("🔒 Autonome BIOS-Thermokurve", "Compal EC regelt PWM dynamisch anhand der internen Temperaturkurve."),
            ("🔒 Courbe thermique BIOS autonome", "Le contrôleur Compal EC gère dynamiquement le PWM selon la courbe."),
            ("🔒 Curva Térmica BIOS Autônoma", "Compal EC gerencia dinamicamente o PWM segundo a curva térmica."),
            ("🔒 Curva termica BIOS autonoma", "Compal EC gestisce dinamicamente il PWM in base alla curva termica."),
            ("🔒 Автономная термокривая BIOS", "Контроллер Compal EC управляет ШИМ по заводской кривой нагрева."),
            ("🔒 BIOS 自律热控动态曲线", "Compal EC 固件接管硬件温控，依据温度阶梯平滑动态调节占空比。"),
            ("🔒 自律的 BIOS サーマルプロファイル", "Compal EC が温度カーブに基づき PWM を自律的に管理します。"),
            ("🔒 BIOS 자율 열 관리 커브", "Compal EC가 발열 커브에 맞춰 PWM을 동적으로 자동 제어합니다."),
        )
    }

    pub fn acoustic_footprint_label(&self) -> &'static str {
        tr(*self,
            "Huella Acústica:", "Acoustic Footprint:", "Akustischer Pegel:", "Niveau sonore :", "Pegada Acústica:",
            "Impronta acustica:", "Акустический шум:", "声学噪声估算:", "騒音レベル目安:", "소음 수준:",
        )
    }

    pub fn acoustic_state(&self, max_rpm: u32) -> (u32, &'static str) {
        if max_rpm == 0 {
            (18, tr(*self,
                "0 RPM • Convección pasiva / Silencio total",
                "0 RPM • Passive Convection / Pure Silence",
                "0 U/min • Passive Konvektion / Völlige Stille",
                "0 tr/min • Convection passive / Silence complet",
                "0 RPM • Convecção passiva / Silêncio total",
                "0 RPM • Convezione passiva / Silenzio totale",
                "0 об/мин • Пассивный режим / Полная тишина",
                "0 RPM • 纯被动对流散热 / 零分贝绝对静音",
                "0 RPM • パッシブ冷却 / 完全静音稼働",
                "0 RPM • 패시브 대류 방열 / 무소음",
            ))
        } else if max_rpm < 2200 {
            (24, tr(*self,
                "Flujo susurro • Casi inaudible / Oficina y dev",
                "Whisper Flow • Near Inaudible / Office & Dev",
                "Flüsterleise • Kaum hörbar / Büro & Entwicklung",
                "Flux silencieux • Pratiquement inaudible / Bureau",
                "Fluxo sussurro • Quase inaudível / Escritório",
                "Flusso silenzioso • Quasi impercettibile / Ufficio",
                "Шелест • Почти неслышно / Офис и разработка",
                "微风轻语 • 几近无声 / 沉浸办公与日常开发",
                "微風フロー • ほぼ無音 / オフィス作業・開発",
                "초저소음 • 거의 무음 / 사무 및 프로그래밍",
            ))
        } else if max_rpm < 3400 {
            (33, tr(*self,
                "Brisa suave • Ligero zumbido ambiente",
                "Gentle Draft • Mild Ambient Whir",
                "Sanfter Luftstrom • Leises Raumsummen",
                "Brise douce • Léger ronronnement ambiant",
                "Brisa suave • Leve zumbido ambiente",
                "Brezza leggera • Ronzio ambientale lieve",
                "Мягкий поток • Легкий фоновый шелест",
                "柔和散热 • 温和气流 / 均衡日常运算",
                "穏やかな気流 • わずかな動作音 / 通常作業",
                "부드러운 미풍 • 가벼운 배경 동작음",
            ))
        } else if max_rpm < 4500 {
            (43, tr(*self,
                "Inducción forzada • Gaming equilibrado audible",
                "Forced Induction • Audible Balanced Gaming",
                "Forcierte Kühlung • Hörbares Gaming",
                "Induction forcée • Gaming audible équilibré",
                "Indução forçada • Jogos equilibrados audíveis",
                "Induzione forzata • Gaming bilanciato udibile",
                "Умеренный наддув • Сбалансированный игровой режим",
                "强制定向风压 • 竞技游戏温控平稳输出",
                "強制冷却 • バランス型ゲーミング",
                "강제 흡기 • 밸런스드 게이밍 및 렌더링",
            ))
        } else if max_rpm < 5400 {
            (51, tr(*self,
                "Alto flujo turbina • Fuerte disipación térmica",
                "High Turbine Flow • Heavy Thermal Dissipation",
                "Hoher Turbinenstrom • Starke Wärmeableitung",
                "Flux élevé • Forte dissipation thermique",
                "Alto fluxo turbina • Forte dissipação térmica",
                "Alto flusso turbine • Forte dissipazione termica",
                "Высокий поток • Мощный отвод тепла",
                "高风压双涡轮 • 强力导流散热应对重度负载",
                "高風量タービン • 強力な熱放散駆動",
                "고유량 터빈 • 강력한 발열 해소 출력",
            ))
        } else {
            (56, tr(*self,
                "Inducción jet máxima • Turbo overdrive 12V total",
                "Max Jet Induction • Full 12V Turbo Overdrive",
                "Maximale Jet-Kühlung • Volle 12V Turbo-Power",
                "Induction jet maximale • Turbo overdrive 12V total",
                "Indução máxima jato • Turbo overdrive 12V total",
                "Induzione massima jet • Turbo overdrive 12V totale",
                "Реактивный наддув • Полный 12V овердрайв",
                "全速喷射气流 • 12V 满血狂暴涡轮压制极值",
                "最大ジェット噴流 • フル12V ターボ過給駆動",
                "최대 제트 기류 • 풀 12V 터보 오버드라이브",
            ))
        }
    }

    pub fn est_airflow_label(&self, cfm: f32) -> String {
        let fmt = tr(*self,
            "Flujo est.: {:.1} CFM",
            "Est. Airflow: {:.1} CFM",
            "Geschätzter Luftstrom: {:.1} CFM",
            "Débit estimé : {:.1} CFM",
            "Fluxo est.: {:.1} CFM",
            "Flusso stimato: {:.1} CFM",
            "Оценка потока: {:.1} CFM",
            "估算气流量: {:.1} CFM",
            "推定風量: {:.1} CFM",
            "예상 풍량: {:.1} CFM",
        );
        fmt.replace("{:.1}", &format!("{:.1}", cfm))
    }

    pub fn ventilation_dynamics_title(&self) -> &'static str {
        tr(*self,
            "Curva de Dinámica de Ventilación",
            "Ventilation Dynamics Curve",
            "Lüftungsdynamik-Kurve",
            "Courbe de dynamique de ventilation",
            "Curva de Dinâmica de Ventilação",
            "Curva dinamica di ventilazione",
            "Кривая динамики вентиляции",
            "双涡轮风道动力学曲线",
            "換気ダイナミクス曲線",
            "환기 역학 커브",
        )
    }

    pub fn dual_turbine_label(&self) -> &'static str {
        tr(*self,
            "• Respuesta Turbinas Duales (60s)",
            "• Dual-Turbine Response (60s)",
            "• Dual-Turbinen-Reaktion (60s)",
            "• Réponse double turbine (60s)",
            "• Resposta de Turbinas Duplas (60s)",
            "• Risposta a doppia turbina (60s)",
            "• Отклик сдвоенных турбин (60с)",
            "• 双涡轮响应实时波形 (60s)",
            "• デュアルタービン応答特性 (60秒)",
            "• 듀얼 터빈 응답 특성 (60초)",
        )
    }

    pub fn telemetry_matrix_title(&self) -> &'static str {
        tr(*self,
            "Matriz de Telemetría del Sistema en Vivo",
            "Live System Telemetry Matrix",
            "Live-System-Telemetriematrix",
            "Matrice de télémétrie système en direct",
            "Matriz de Telemetria do Sistema em Tempo Real",
            "Matrice di telemetria di sistema in tempo reale",
            "Матрица телеметрии оборудования в реальном времени",
            "全系统实时硬件遥测指标矩阵",
            "リアルタイム システム テレメトリ マトリックス",
            "실시간 시스템 하드웨어 텔레메트리 매트릭스",
        )
    }

    // --- MONITORING TAB ---
    pub fn kpi_thermal_headroom(&self) -> &'static str {
        tr(*self,
            "MARGEN TÉRMICO", "THERMAL HEADROOM", "THERMISCHER SPIELRAUM", "MARGE THERMIQUE", "MARGEM TÉRMICA",
            "MARGINE TERMICO", "ТЕПЛОВОЙ ЗАПАС", "散热安全余量", "サーマルヘッドルーム", "열 마진 여유도",
        )
    }

    pub fn kpi_peak_temps(&self, cpu: f32, gpu: f32) -> String {
        let fmt = tr(*self,
            "Pico: {:.0}°C CPU / {:.0}°C GPU",
            "Peak: {:.0}°C CPU / {:.0}°C GPU",
            "Spitze: {:.0}°C CPU / {:.0}°C GPU",
            "Pic : {:.0}°C CPU / {:.0}°C GPU",
            "Pico: {:.0}°C CPU / {:.0}°C GPU",
            "Picco: {:.0}°C CPU / {:.0}°C GPU",
            "Пик: {:.0}°C CPU / {:.0}°C GPU",
            "峰值: {:.0}°C CPU / {:.0}°C GPU",
            "ピーク: {:.0}°C CPU / {:.0}°C GPU",
            "피크: {:.0}°C CPU / {:.0}°C GPU",
        );
        let s = fmt.replacen("{:.0}", &format!("{:.0}", cpu), 1);
        s.replacen("{:.0}", &format!("{:.0}", gpu), 1)
    }

    pub fn kpi_combined_power(&self) -> &'static str {
        tr(*self,
            "POTENCIA COMBINADA", "COMBINED POWER", "GESAMTLEISTUNG", "PUISSANCE COMBINÉE", "POTÊNCIA COMBINADA",
            "POTENZA COMBINATA", "СУММАРНАЯ МОЩНОСТЬ", "整机综合功耗", "合計消費電力", "통합 소비 전력",
        )
    }

    pub fn kpi_peak_power(&self, p: f32) -> String {
        let fmt = tr(*self,
            "Pico: {:.1} W", "Peak: {:.1} W", "Spitze: {:.1} W", "Pic : {:.1} W", "Pico: {:.1} W",
            "Picco: {:.1} W", "Пик: {:.1} W", "峰值功耗: {:.1} W", "最大電力: {:.1} W", "최대 전력: {:.1} W",
        );
        fmt.replace("{:.1}", &format!("{:.1}", p))
    }

    pub fn kpi_peak_core(&self) -> &'static str {
        tr(*self,
            "TEMP MÁX NÚCLEO", "PEAK CORE TEMP", "MAX KERNTEMPERATUR", "TEMP MAX CŒUR", "TEMP MÁX NÚCLEO",
            "TEMP MAX CORE", "МАКС ТЕМП ЯДРА", "核心最高峰值温度", "最大コア温度", "최대 코어 온도",
        )
    }

    pub fn kpi_tjmax(&self) -> &'static str {
        tr(*self,
            "Referencia TjMax: 100°C", "TjMax Reference: 100°C", "TjMax-Referenz: 100°C", "Référence TjMax : 100°C", "Referência TjMax: 100°C",
            "Riferimento TjMax: 100°C", "Ориентир TjMax: 100°C", "TjMax 额定参考阈值: 100°C", "TjMax 基準値: 100°C", "TjMax 기준치: 100°C",
        )
    }

    pub fn kpi_turbine_utilization(&self) -> &'static str {
        tr(*self,
            "USO DE TURBINAS", "TURBINE UTILIZATION", "TURBINEN-AUSLASTUNG", "UTILISATION TURBINES", "USO DE TURBINAS",
            "UTILIZZO TURBINE", "НАГРУЗКА ТУРБИН", "涡轮风扇总转速负荷", "ファン稼働率", "터빈 팬 가동률",
        )
    }

    pub fn kpi_dynamic_saturation(&self) -> &'static str {
        tr(*self,
            "Saturación dinámica", "Dynamic Saturation", "Dynamische Sättigung", "Saturation dynamique", "Saturação dinâmica",
            "Saturazione dinamica", "Динамическое насыщение", "动态饱和度", "動的飽和率", "동적 포화도",
        )
    }

    pub fn graph_thermals(&self) -> &'static str {
        tr(*self,
            "🌡 Térmicas", "🌡 Thermals", "🌡 Temperaturen", "🌡 Températures", "🌡 Térmicas",
            "🌡 Termiche", "🌡 Температуры", "🌡 温度曲线", "🌡 温度", "🌡 온도",
        )
    }

    pub fn graph_workload(&self) -> &'static str {
        tr(*self,
            "📈 Carga", "📈 Workload", "📈 Auslastung", "📈 Charge", "📈 Carga",
            "📈 Carico", "📈 Нагрузка", "📈 硬件负载", "📈 負荷率", "📈 부하율",
        )
    }

    pub fn graph_power(&self) -> &'static str {
        tr(*self,
            "⚡ Potencia", "⚡ Power", "⚡ Leistung", "⚡ Puissance", "⚡ Potência",
            "⚡ Potenza", "⚡ Мощность", "⚡ 动态功耗", "⚡ 電力", "⚡ 전력",
        )
    }

    pub fn graph_turbines(&self) -> &'static str {
        tr(*self,
            "🌀 Turbinas", "🌀 Turbines", "🌀 Turbinen", "🌀 Turbines", "🌀 Turbinas",
            "🌀 Turbine", "🌀 Турбины", "🌀 涡轮转速", "🌀 ファン回転", "🌀 터빈",
        )
    }

    pub fn graph_pause(&self) -> &'static str {
        tr(*self,
            "⏸ Pausar", "⏸ Pause", "⏸ Anhalten", "⏸ Pause", "⏸ Pausar",
            "⏸ Pausa", "⏸ Пауза", "⏸ 冻结采样", "⏸ 一時停止", "⏸ 일시정지",
        )
    }

    pub fn graph_resume(&self) -> &'static str {
        tr(*self,
            "▶ Reanudar", "▶ Resume", "▶ Fortsetzen", "▶ Reprendre", "▶ Retomar",
            "▶ Riprendi", "▶ Продолжить", "▶ 恢复捕获", "▶ 再開", "▶ 재개",
        )
    }

    pub fn graph_clear(&self) -> &'static str {
        tr(*self,
            "🗑 Limpiar", "🗑 Clear", "🗑 Leeren", "🗑 Effacer", "🗑 Limpar",
            "🗑 Cancella", "🗑 Очистить", "🗑 清空历史", "🗑 消去", "🗑 지우기",
        )
    }

    pub fn card_cpu_title(&self) -> &'static str {
        tr(*self,
            "PAQUETE CPU", "CPU PACKAGE", "CPU-PAKET", "PROCESSEUR CPU", "PACOTE CPU",
            "PACCHETTO CPU", "ПРОЦЕССОР (CPU)", "中央处理器 (CPU)", "CPU パッケージ", "CPU 패키지",
        )
    }

    pub fn card_gpu_title(&self) -> &'static str {
        tr(*self,
            "PROCESADOR GRÁFICO (GPU)", "GRAPHICS PROCESSING UNIT (GPU)", "GRAFIKPROZESSOR (GPU)", "PROCESSEUR GRAPHIQUE (GPU)", "UNIDADE GRÁFICA (GPU)",
            "PROCESSORE GRAFICO (GPU)", "ГРАФИЧЕСКИЙ ЧИП (GPU)", "图形处理器 (GPU)", "グラフィックス (GPU)", "그래픽 프로세서 (GPU)",
        )
    }

    pub fn card_memory_title(&self) -> &'static str {
        tr(*self,
            "MEMORIA DEL SISTEMA (RAM)", "SYSTEM MEMORY (RAM)", "ARBEITSSPEICHER (RAM)", "MÉMOIRE SYSTÈME (RAM)", "MEMÓRIA DO SISTEMA (RAM)",
            "MEMORIA DI SISTEMA (RAM)", "ОПЕРАТИВНАЯ ПАМЯТЬ (RAM)", "系统物理内存 (RAM)", "システムメモリ (RAM)", "시스템 메모리 (RAM)",
        )
    }

    pub fn card_storage_title(&self) -> &'static str {
        tr(*self,
            "ALMACENAMIENTO NVME", "NVME SOLID STATE STORAGE", "NVME-FESTPLATTE", "STOCKAGE SSD NVME", "ARMAZENAMENTO NVME",
            "ARCHIVIAZIONE SSD NVME", "НАКОПИТЕЛЬ NVME SSD", "NVME 固态硬盘存储", "NVME 高速ストレージ", "NVME SSD 스토리지",
        )
    }

    pub fn card_power_title(&self) -> &'static str {
        tr(*self,
            "ALIMENTACIÓN Y BATERÍA", "POWER DELIVERY & CHARGING", "STROMVERSORGUNG & AKKU", "ALIMENTATION ET BATTERIE", "ALIMENTAÇÃO E BATERIA",
            "ALIMENTAZIONE E BATTERIA", "ПИТАНИЕ И БАТАРЕЯ", "电源适配器与电池管理", "電源供給＆バッテリー", "전원 공급 및 배터리",
        )
    }

    pub fn card_turbines_title(&self) -> &'static str {
        tr(*self,
            "TURBINAS AERODINÁMICAS", "AERODYNAMIC TURBINES", "AERODYNAMISCHE TURBINEN", "TURBINES AÉRODYNAMIQUES", "TURBINAS AERODINÂMICAS",
            "TURBINE AERODINAMICHE", "АЭРОДИНАМИЧЕСКИЕ ТУРБИНЫ", "AEROBLADE 3D 动力学涡轮", "空力タービンブレード", "공기역학 터빈 블레이드",
        )
    }

    // --- SCENARIOS (POWER MODES) TAB ---
    pub fn scenarios_header(&self) -> &'static str {
        tr(*self,
            "⚡ Escenarios Operativos y Perfiles de Potencia",
            "⚡ Operating Scenarios & Power Envelopes",
            "⚡ Betriebsszenarien & Leistungsprofile",
            "⚡ Scénarios opérationnels et profils énergétiques",
            "⚡ Cenários Operacionais e Perfis de Energia",
            "⚡ Scenari operativi e profili di potenza",
            "⚡ Сценарии работы и профили энергопотребления",
            "⚡ Acer Nitro 性能运行场景与功耗墙调度",
            "⚡ 動作シナリオ＆電力エンベロープ制御",
            "⚡ 동작 시나리오 및 전력 프로필 관리",
        )
    }

    pub fn current_badge(&self, current: &str) -> String {
        let fmt = tr(*self,
            "ACTIVO: {}", "CURRENT: {}", "AKTIV: {}", "ACTIF : {}", "ATIVO: {}",
            "ATTIVO: {}", "АКТИВЕН: {}", "当前生效: {}", "現在有効: {}", "현재 적용: {}",
        );
        fmt.replace("{}", &current.to_uppercase())
    }

    pub fn scenario_items(&self) -> [(&'static str, &'static str, &'static str, &'static str); 4] {
        match self {
            Self::Es => [
                ("quiet", "🌱 Silencioso / Eco Sigiloso", "Máxima autonomía de batería y silencio acústico (<22 dBA). Limita CPU a 15W TDP y ventiladores a velocidad susurro.", "PL1: 15W | Turbinas: Silencioso Min | Temp: Frío"),
                ("balanced", "⚡ Rendimiento Equilibrado", "Configuración diaria óptima para desarrollo, auditorías y refrigeración adaptativa.", "PL1: 45W | Turbinas: Auto BIOS | Temp: Nominal"),
                ("performance", "🔥 Rendimiento Gaming", "Frecuencias de CPU desbloqueadas (hasta 4.50 GHz boost) con curva acelerada para multitarea pesada.", "PL1: 45W+ | CoolBoost: Activo | Temp: Activo"),
                ("turbo", "🚀 Combate Extremo Turbo", "Saturación completa al 100% PWM (5660/6000 RPM) con máximo envolvente PL2 para cracking y estrés.", "PL2: 65W | GPU TGP: 75W | Turbinas: 100% Turbo"),
            ],
            Self::En => [
                ("quiet", "🌱 Quiet / Eco Stealth", "Maximum battery endurance & acoustic silence (<22 dBA). Limits CPU to 15W TDP and fans at whisper speed.", "PL1: 15W | Fan: Silent Min | Temp: Cold"),
                ("balanced", "⚡ Balanced Performance", "Optimal daily configuration for development, audits, and adaptive cooling under varying loads.", "PL1: 45W | Fan: Auto BIOS | Temp: Nominal"),
                ("performance", "🔥 Performance Gaming", "Unlocked CPU clock speeds (up to 4.50 GHz boost) with accelerated cooling curve for heavy loads.", "PL1: 45W+ | CoolBoost: Active | Temp: Active"),
                ("turbo", "🚀 Extreme Combat Turbo", "Full 100% PWM fan saturation (5660/6000 RPM) with maximum PL2 envelope for heavy stress loads.", "PL2: 65W | GPU TGP: 75W | Fan: 100% Turbo"),
            ],
            Self::De => [
                ("quiet", "🌱 Leise / Eco Stealth", "Maximale Akkulaufzeit & Stille (<22 dBA). Begrenzt CPU auf 15W TDP bei Flüsterdrehzahl.", "PL1: 15W | Lüfter: Flüsterleise | Temp: Kühl"),
                ("balanced", "⚡ Ausgewogene Leistung", "Optimale Konfiguration für tägliche Entwicklung und adaptive Lüftung.", "PL1: 45W | Lüfter: Auto BIOS | Temp: Nominal"),
                ("performance", "🔥 Gaming-Leistung", "Erhöhte CPU-Taktung mit aggressiver Lüfterkurve für anspruchsvolles Multitasking.", "PL1: 45W+ | CoolBoost: Aktiv | Temp: Aktiv"),
                ("turbo", "🚀 Extrem-Turbo", "100% PWM-Lüftervollgas (5660/6000 U/min) mit maximaler PL2-Leistung für Höchstlast.", "PL2: 65W | GPU TGP: 75W | Lüfter: 100% Turbo"),
            ],
            Self::Fr => [
                ("quiet", "🌱 Silencieux / Éco Discret", "Autonomie max et silence (<22 dBA). Limite le CPU à 15W TDP avec ventilation minimale.", "PL1: 15W | Ventilateurs : Discret | Temp : Froid"),
                ("balanced", "⚡ Performance Équilibrée", "Configuration quotidienne idéale pour le dev et la navigation fluide.", "PL1: 45W | Ventilateurs : Auto BIOS | Temp : Nominal"),
                ("performance", "🔥 Performance Jeu", "Fréquences boost libérées avec refroidissement renforcé pour charges lourdes.", "PL1: 45W+ | CoolBoost : Actif | Temp : Actif"),
                ("turbo", "🚀 Combat Extrême Turbo", "Ventilation à 100% PWM (5660/6000 tr/min) avec enveloppe PL2 maximale.", "PL2: 65W | GPU TGP : 75W | Ventilateurs : 100%"),
            ],
            Self::Pt => [
                ("quiet", "🌱 Silencioso / Eco Furtivo", "Máxima autonomia de bateria e silêncio (<22 dBA). Limita CPU a 15W TDP e ventoinhas silenciosas.", "PL1: 15W | Ventoinhas: Mínimo | Temp: Frio"),
                ("balanced", "⚡ Desempenho Equilibrado", "Configuração diária ideal para desenvolvimento e refrigeração adaptativa.", "PL1: 45W | Ventoinhas: Auto BIOS | Temp: Nominal"),
                ("performance", "🔥 Desempenho Jogos", "Frequências boost liberadas com curva acelerada para multitarefa pesada.", "PL1: 45W+ | CoolBoost: Ativo | Temp: Ativo"),
                ("turbo", "🚀 Combate Extremo Turbo", "Saturação 100% PWM (5660/6000 RPM) com limite de energia PL2 máximo.", "PL2: 65W | GPU TGP: 75W | Ventoinhas: 100%"),
            ],
            Self::It => [
                ("quiet", "🌱 Silenzioso / Eco Silente", "Massima autonomia e silenzio (<22 dBA). Limita la CPU a 15W TDP.", "PL1: 15W | Ventole: Minimo | Temp: Freddo"),
                ("balanced", "⚡ Prestazioni Bilanciate", "Profilo ideale per lavoro quotidiano e raffreddamento adattivo.", "PL1: 45W | Ventole: Auto BIOS | Temp: Nominale"),
                ("performance", "🔥 Prestazioni Gaming", "Frequenze sbloccate con curva termica aggressiva per carichi pesanti.", "PL1: 45W+ | CoolBoost: Attivo | Temp: Attivo"),
                ("turbo", "🚀 Turbo Estremo", "Massima saturazione 100% PWM (5660/6000 RPM) con PL2 massimo.", "PL2: 65W | GPU TGP: 75W | Ventole: 100% Turbo"),
            ],
            Self::Ru => [
                ("quiet", "🌱 Тихий / Эко-стелс", "Максимальная автономность и тишина (<22 дБА). Ограничение CPU 15W TDP.", "PL1: 15W | Кулеры: Тихий мин. | Темп: Холод"),
                ("balanced", "⚡ Сбалансированный", "Оптимальный ежедневный режим для разработки и повседневных задач.", "PL1: 45W | Кулеры: Авто BIOS | Темп: Норма"),
                ("performance", "🔥 Игровая производительность", "Повышенные частоты CPU и ускоренное охлаждение для тяжелых нагрузок.", "PL1: 45W+ | CoolBoost: Вкл | Темп: Актив"),
                ("turbo", "🚀 Экстремальный Турбо", "100% ШИМ вентиляторов (5660/6000 об/мин) и полный лимит мощности PL2.", "PL2: 65W | GPU TGP: 75W | Кулеры: 100% Турбо"),
            ],
            Self::Zh => [
                ("quiet", "🌱 极致静音 / 节能隐航", "最大限度延长电池续航并消除风噪 (<22 dBA)，CPU 功耗锁 15W TDP。", "PL1: 15W | 涡轮: 微风轻音 | 温度: 冰凉"),
                ("balanced", "⚡ 均衡性能 / 日常全能", "出厂默认最佳平衡点，智能适应日常研发与多任务负载调校。", "PL1: 45W | 涡轮: BIOS 自适应 | 温度: 额定"),
                ("performance", "🔥 狂暴竞技 / 游戏释放", "激进拉升 CPU 睿频，结合主动增强型热阻曲线无惧瞬态高负载。", "PL1: 45W+ | CoolBoost: 激活 | 温度: 活跃"),
                ("turbo", "🚀 战备极限 / 极速超频", "双涡轮全速 100% PWM (5660/6000 RPM) 满血运转，解锁极致功耗墙。", "PL2: 65W | GPU TGP: 75W | 涡轮: 100% 狂暴"),
            ],
            Self::Ja => [
                ("quiet", "🌱 静音 / エコステルス", "バッテリー持続最優先＆静音動作 (<22 dBA)。CPU を 15W TDP に抑制。", "PL1: 15W | ファン: 最低回転 | 温度: 低温"),
                ("balanced", "⚡ バランス パフォーマンス", "開発作業や日常ユースに最適な自動適応冷却設定。", "PL1: 45W | ファン: 自動 BIOS | 温度: 標準"),
                ("performance", "🔥 ゲーミング パフォーマンス", "ブーストクロックを解放し、高負荷に備えた冷却カーブを適用。", "PL1: 45W+ | CoolBoost: 有効 | 温度: 積極冷却"),
                ("turbo", "🚀 エクストリーム ターボ", "100% PWM 全開 (5660/6000 RPM) で限界まで冷却し、最高性能を維持。", "PL2: 65W | GPU TGP: 75W | ファン: 100% ターボ"),
            ],
            Self::Ko => [
                ("quiet", "🌱 저소음 / 에코 스텔스", "최대 배터리 절전 및 무소음 환경 (<22 dBA). CPU TDP 15W 제한.", "PL1: 15W | 팬: 최저속 | 온도: 쿨링"),
                ("balanced", "⚡ 밸런스드 퍼포먼스", "소프트웨어 개발 및 일상 작업을 위한 최적의 균형 모드.", "PL1: 45W | 팬: BIOS 자동 | 온도: 표준"),
                ("performance", "🔥 게이밍 퍼포먼스", "CPU 부스트 클럭을 개방하고 냉각 커브를 가속하여 고성능 유지.", "PL1: 45W+ | CoolBoost: 활성 | 온도: 고성능"),
                ("turbo", "🚀 익스트림 터보", "팬 100% 풀 PWM 가동 (5660/6000 RPM) 및 최대 PL2 전력 한계치 개방.", "PL2: 65W | GPU TGP: 75W | 팬: 100% 터보"),
            ],
        }
    }

    pub fn active_indicator(&self) -> &'static str {
        tr(*self,
            "● ACTIVO", "● ACTIVE", "● AKTIV", "● ACTIF", "● ATIVO",
            "● ATTIVO", "● АКТИВЕН", "● 当前激活", "● 有効", "● 활성",
        )
    }

    pub fn engage_button(&self) -> &'static str {
        tr(*self,
            "Activar", "Engage", "Aktivieren", "Activer", "Ativar",
            "Attiva", "Применить", "应用生效", "適用", "적용",
        )
    }

    // --- KEYBOARD RGB TAB ---
    pub fn rgb_header(&self) -> &'static str {
        tr(*self,
            "🌈 Estudio de Iluminación de Teclado Pulsar 4-Zonas",
            "🌈 4-Zone Pulsar Keyboard Lighting Studio",
            "🌈 4-Zonen-Pulsar-Tastaturbeleuchtungsstudio",
            "🌈 Studio d'éclairage clavier Pulsar 4 zones",
            "🌈 Estúdio de Iluminação de Teclado Pulsar 4 Zonas",
            "🌈 Studio di illuminazione tastiera Pulsar a 4 zone",
            "🌈 Студия 4-зонной RGB-подсветки клавиатуры Pulsar",
            "🌈 Acer Nitro Pulsar 四区独立 RGB 键盘灯效工坊",
            "🌈 4ゾーン Pulsar キーボード RGB ライティング スタジオ",
            "🌈 4-구역 Pulsar 키보드 RGB 백라이트 스튜디오",
        )
    }

    pub fn select_zone_label(&self) -> &'static str {
        tr(*self,
            "Selecciona la Zona del Teclado:",
            "Select Target Keyboard Zone:",
            "Tastaturzone auswählen:",
            "Sélectionner la zone du clavier :",
            "Selecione a Zona do Teclado:",
            "Seleziona la zona della tastiera:",
            "Выберите зону клавиатуры:",
            "选择目标键盘发光分区:",
            "キーボードゾーンを選択:",
            "대상 키보드 구역 선택:",
        )
    }

    pub fn rgb_zones(&self) -> [(u8, &'static str); 5] {
        match self {
            Self::Es => [
                (0, "🌐 Todas las Zonas (1-4)"),
                (1, "🎮 Zona 1 (WASD)"),
                (2, "⌨ Zona 2 (Centro-Izq)"),
                (3, "⌨ Zona 3 (Centro-Der)"),
                (4, "🔢 Zona 4 (NumPad)"),
            ],
            Self::En => [
                (0, "🌐 All Zones (1-4)"),
                (1, "🎮 Zone 1 (WASD)"),
                (2, "⌨ Zone 2 (Center-Left)"),
                (3, "⌨ Zone 3 (Center-Right)"),
                (4, "🔢 Zone 4 (NumPad)"),
            ],
            Self::De => [
                (0, "🌐 Alle Zonen (1-4)"),
                (1, "🎮 Zone 1 (WASD)"),
                (2, "⌨ Zone 2 (Mitte-Links)"),
                (3, "⌨ Zone 3 (Mitte-Rechts)"),
                (4, "🔢 Zone 4 (Ziffernblock)"),
            ],
            Self::Fr => [
                (0, "🌐 Toutes les zones (1-4)"),
                (1, "🎮 Zone 1 (WASD)"),
                (2, "⌨ Zone 2 (Centre-Gauche)"),
                (3, "⌨ Zone 3 (Centre-Droit)"),
                (4, "🔢 Zone 4 (Pavé numérique)"),
            ],
            Self::Pt => [
                (0, "🌐 Todas as Zonas (1-4)"),
                (1, "🎮 Zona 1 (WASD)"),
                (2, "⌨ Zona 2 (Centro-Esq)"),
                (3, "⌨ Zona 3 (Centro-Dir)"),
                (4, "🔢 Zona 4 (Teclado Numérico)"),
            ],
            Self::It => [
                (0, "🌐 Tutte le zone (1-4)"),
                (1, "🎮 Zona 1 (WASD)"),
                (2, "⌨ Zona 2 (Centro-Sinistra)"),
                (3, "⌨ Zona 3 (Centro-Destra)"),
                (4, "🔢 Zona 4 (Tastierino)"),
            ],
            Self::Ru => [
                (0, "🌐 Все зоны (1-4)"),
                (1, "🎮 Зона 1 (WASD)"),
                (2, "⌨ Зона 2 (Центр-Слева)"),
                (3, "⌨ Зона 3 (Центр-Справа)"),
                (4, "🔢 Зона 4 (NumPad)"),
            ],
            Self::Zh => [
                (0, "🌐 全局背光同步 (分区 1-4)"),
                (1, "🎮 分区 1 (WASD 游戏竞技区)"),
                (2, "⌨ 分区 2 (主打字区左翼)"),
                (3, "⌨ 分区 3 (主打字区右翼)"),
                (4, "🔢 分区 4 (数字小键盘区)"),
            ],
            Self::Ja => [
                (0, "🌐 全ゾーン同期 (1-4)"),
                (1, "🎮 ゾーン 1 (WASD ゲーミング)"),
                (2, "⌨ ゾーン 2 (中央・左)"),
                (3, "⌨ ゾーン 3 (中央・右)"),
                (4, "🔢 ゾーン 4 (テンキー)"),
            ],
            Self::Ko => [
                (0, "🌐 전체 구역 연동 (1-4)"),
                (1, "🎮 구역 1 (WASD 게이밍)"),
                (2, "⌨ 구역 2 (중앙 좌측)"),
                (3, "⌨ 구역 3 (중앙 우측)"),
                (4, "🔢 구역 4 (숫자 패드)"),
            ],
        }
    }

    pub fn static_color_title(&self, target_desc: &str) -> String {
        let fmt = tr(*self,
            "Color Estático para {}:",
            "Static Color for {}:",
            "Statische Farbe für {}:",
            "Couleur statique pour {} :",
            "Cor Estática para {}:",
            "Colore statico per {}:",
            "Статический цвет для {}:",
            "{} 静态色彩设定:",
            "{} の固定カラー:",
            "{} 단색 컬러 설정:",
        );
        fmt.replace("{}", target_desc)
    }

    pub fn apply_zone_color(&self) -> &'static str {
        tr(*self,
            "Aplicar Color a Zona", "Apply Zone Color", "Farbe auf Zone anwenden", "Appliquer à la zone", "Aplicar Cor à Zona",
            "Applica colore alla zona", "Применить к зоне", "写入所选分区", "ゾーンに色を適用", "구역에 색상 적용",
        )
    }

    pub fn dynamic_effects_title(&self) -> &'static str {
        tr(*self,
            "Efectos Dinámicos y Presets:", "Dynamic Lighting Effects & Presets:", "Dynamische Effekte & Profile:", "Effets dynamiques et préréglages :", "Efeitos Dinâmicos e Predefinições:",
            "Effetti dinamici e preset:", "Динамические эффекты и пресеты:", "动态流光灯效与专属预设:", "ダイナミック発光エフェクト＆プリセット:", "동적 조명 효과 및 프리셋:",
        )
    }

    pub fn rgb_presets(&self) -> [(&'static str, &'static str); 6] {
        match self {
            Self::Es => [
                ("static_red", "Estático Nitro Rojo"),
                ("static_cyan", "Cian Ártico"),
                ("static_green", "Verde Terminal"),
                ("rainbow", "Ciclo Arcoíris"),
                ("breathing", "Ola Respiración"),
                ("off", "Apagar Luces"),
            ],
            Self::En => [
                ("static_red", "Static Nitro Red"),
                ("static_cyan", "Arctic Cyan"),
                ("static_green", "Terminal Green"),
                ("rainbow", "Rainbow Cycle"),
                ("breathing", "Breathing Wave"),
                ("off", "Lights Off"),
            ],
            Self::De => [
                ("static_red", "Statisches Nitro-Rot"),
                ("static_cyan", "Arktisches Cyan"),
                ("static_green", "Terminal-Grün"),
                ("rainbow", "Regenbogen-Zyklus"),
                ("breathing", "Atmende Welle"),
                ("off", "Beleuchtung Aus"),
            ],
            Self::Fr => [
                ("static_red", "Rouge Nitro Statique"),
                ("static_cyan", "Cyan Arctique"),
                ("static_green", "Vert Terminal"),
                ("rainbow", "Cycle Arc-en-ciel"),
                ("breathing", "Respiration Douce"),
                ("off", "Éteindre les lumières"),
            ],
            Self::Pt => [
                ("static_red", "Vermelho Nitro Estático"),
                ("static_cyan", "Ciano Ártico"),
                ("static_green", "Verde Terminal"),
                ("rainbow", "Ciclo Arco-íris"),
                ("breathing", "Onda de Respiração"),
                ("off", "Desligar Luzes"),
            ],
            Self::It => [
                ("static_red", "Rosso Nitro Statico"),
                ("static_cyan", "Ciano Artico"),
                ("static_green", "Verde Terminale"),
                ("rainbow", "Ciclo Arcobaleno"),
                ("breathing", "Respiro Luminoso"),
                ("off", "Spegni luci"),
            ],
            Self::Ru => [
                ("static_red", "Статический красный Nitro"),
                ("static_cyan", "Арктический бирюзовый"),
                ("static_green", "Терминальный зеленый"),
                ("rainbow", "Радужный цикл"),
                ("breathing", "Дыхание волной"),
                ("off", "Выключить подсветку"),
            ],
            Self::Zh => [
                ("static_red", "战意炽红 (Nitro Red)"),
                ("static_cyan", "极光青蓝 (Arctic Cyan)"),
                ("static_green", "黑客苍翠 (Terminal Green)"),
                ("rainbow", "幻彩七色霓虹巡航"),
                ("breathing", "柔光呼吸律动循环"),
                ("off", "完全关闭键盘灯效"),
            ],
            Self::Ja => [
                ("static_red", "スタティック・ニトロレッド"),
                ("static_cyan", "アークティック・シアン"),
                ("static_green", "ターミナル・グリーン"),
                ("rainbow", "レインボー・サイクル"),
                ("breathing", "ブリージング・ウェーブ"),
                ("off", "バックライト消灯"),
            ],
            Self::Ko => [
                ("static_red", "정적 니트로 레드"),
                ("static_cyan", "북극 시안"),
                ("static_green", "터미널 그린"),
                ("rainbow", "레인보우 사이클"),
                ("breathing", "브리딩 웨이브"),
                ("off", "조명 끄기"),
            ],
        }
    }

    pub fn brightness_label(&self) -> &'static str {
        tr(*self,
            "Brillo:", "Brightness:", "Helligkeit:", "Luminosité :", "Brilho:",
            "Luminosità:", "Яркость:", "背光亮度调节:", "輝度:", "밝기:",
        )
    }

    // --- SETTINGS TAB ---
    pub fn settings_lang_header(&self) -> &'static str {
        tr(*self,
            "🌐 Idioma de la Interfaz / Interface Language",
            "🌐 Interface Language / Idioma de la Interfaz",
            "🌐 Sprache der Benutzeroberfläche",
            "🌐 Langue de l'interface",
            "🌐 Idioma da Interface",
            "🌐 Lingua dell'interfaccia",
            "🌐 Язык интерфейса",
            "🌐 界面多语言国际化设置 (Language)",
            "🌐 インターフェース言語設定",
            "🌐 인터페이스 언어 설정",
        )
    }

    pub fn settings_lang_subtitle(&self) -> &'static str {
        tr(*self,
            "Selecciona tu idioma preferido entre los 10 principales o presiona [L] para alternar en vivo.",
            "Select your preferred language among the top 10 or press [L] anytime to hot-switch instantly.",
            "Wähle deine bevorzugte Sprache aus den Top 10 oder drücke [L] zum schnellen Umschalten.",
            "Sélectionnez votre langue parmi les 10 principales ou appuyez sur [L] pour basculer en direct.",
            "Selecione seu idioma preferido entre os 10 principais ou pressione [L] para alternar em tempo real.",
            "Seleziona la tua lingua preferita tra le 10 principali o premi [L] per cambiare all'istante.",
            "Выберите предпочитаемый язык из топ-10 или нажмите [L] для быстрого переключения.",
            "从全球 10 大主流语言中任意点选切换，或在任何界面直接按 [L] 键即时热切换生效。",
            "主要10言語から選択、または [L] キーでいつでも瞬時に切り替えが可能です。",
            "10대 주요 언어 중 원하는 언어를 선택하거나 [L] 키를 눌러 즉시 전환할 수 있습니다.",
        )
    }

    pub fn settings_theme_header(&self) -> &'static str {
        tr(*self,
            "🎨 Paleta de Color & Ergonomía Visual",
            "🎨 Color Palette & Visual Ergonomics",
            "🎨 Farbpalette & Visuelle Ergonomie",
            "🎨 Palette de couleurs et ergonomie visuelle",
            "🎨 Paleta de Cores e Ergonomia Visual",
            "🎨 Palette colori ed ergonomia visiva",
            "🎨 Цветовая палитра и зрительная эргономика",
            "🎨 视觉人体工学与护眼高保真色彩主题",
            "🎨 カラーパレット＆視覚エルゴノミクス",
            "🎨 컬러 팔레트 및 시각 인체공학 테마",
        )
    }

    pub fn settings_theme_subtitle(&self) -> &'static str {
        tr(*self,
            "Diseño optimizado para confort humano, reduciendo el estrés retinal en sesiones prolongadas.",
            "Optimized for human comfort, reducing retinal strain and enhancing technical legibility during extended sessions.",
            "Optimiert für Sehkomfort und ermüdungsfreie Ablesbarkeit bei langen Einsätzen.",
            "Optimisé pour le confort visuel, réduisant la fatigue oculaire lors de longues sessions.",
            "Design otimizado para conforto visual, reduzindo a fadiga retinal em sessões prolongadas.",
            "Design ottimizzato per il comfort visivo, riducendo l'affaticamento della retina in lunghe sessioni.",
            "Дизайн оптимизирован для защиты зрения и снижения усталости глаз при долгой работе.",
            "针对长时间高强度运维调优，显著消除视网膜蓝光光敏疲劳，提供 25 种舒适配色方案。",
            "長時間の作業でも目の疲労を低減し、高い視認性を保つ人間工学設計。",
            "장시간 작업 시 눈의 피로를 최소화하고 가독성을 극대화하도록 최적화된 설계.",
        )
    }

    pub fn select_button(&self) -> &'static str {
        tr(*self,
            "Seleccionar", "Select", "Auswählen", "Sélectionner", "Selecionar",
            "Seleziona", "Выбрать", "选用此主题", "選択", "선택",
        )
    }

    pub fn active_theme_badge(&self) -> &'static str {
        tr(*self,
            "✔ ACTIVO", "✔ ACTIVE", "✔ AKTIV", "✔ ACTIF", "✔ ATIVO",
            "✔ ATTIVO", "✔ АКТИВНО", "✔ 已生效", "✔ 有効", "✔ 적용됨",
        )
    }

    pub fn battery_care_header(&self) -> &'static str {
        tr(*self,
            "🔋 Cuidado de Batería y Sistema de Litio",
            "🔋 Battery & Lithium Care System",
            "🔋 Akkupflege & Lithium-Schutz",
            "🔋 Entretien de la batterie et protection lithium",
            "🔋 Cuidados com a Bateria e Sistema de Lítio",
            "🔋 Cura della batteria e sistema al litio",
            "🔋 Защита и долговечность литиевой батареи",
            "🔋 锂电池寿命养护与硬件充电阈值管理",
            "🔋 バッテリー保護＆リチウム劣化防止",
            "🔋 배터리 수명 관리 및 충전 제한 시스템",
        )
    }

    pub fn battery_limiter_checkbox(&self) -> &'static str {
        tr(*self,
            "Limitador de Protección de Salud de Batería al 80%",
            "80% Battery Health Protection Limiter",
            "80% Akku-Ladeschutzgrenze",
            "Limiteur de protection de santé de batterie à 80%",
            "Limitador de Proteção de Saúde da Bateria a 80%",
            "Limitatore di protezione salute batteria all'80%",
            "Ограничение заряда до 80% для сохранения ресурса",
            "开启 80% 硬件级防鼓包健康充电阈值限制",
            "バッテリー寿命保護 80% 充電制限",
            "배터리 수명 보호 80% 충전 제한 활성화",
        )
    }

    pub fn battery_limiter_desc(&self) -> &'static str {
        tr(*self,
            "Detiene la carga al ~80% de capacidad para prevenir la degradación química y la hinchazón de celdas bajo uso continuo conectado a la red eléctrica.",
            "Stops charging at ~80% capacity to prevent chemical degradation and battery swelling during continuous AC mains usage.",
            "Stoppt den Ladevorgang bei ~80%, um chemische Zellalterung und Aufblähen bei dauerhafter Netzversorgung zu verhindern.",
            "Arrête la charge à ~80% pour prévenir la dégradation chimique et le gonflement des cellules sur secteur continu.",
            "Interrompe a carga em ~80% para evitar degradação química e inchaço das células em uso contínuo na tomada.",
            "Interrompe la ricarica all'80% circa per prevenire il deterioramento chimico e il rigonfiamento delle celle sotto alimentazione continua.",
            "Останавливает зарядку на ~80%, предотвращая деградацию лития и вздутие банок при постоянном питании от сети.",
            "当设备长期插电使用时，将充电截止电压锁定在约 80% 容量，从根本上防止电极析锂与电芯高温胀气老化。",
            "AC電源接続時の常時充電によるセルの膨張や化学的劣化を防ぐため、充電を約80%で停止します。",
            "상시 전원 연결 사용 시 배터리 셀의 화학적 열화와 스웰링(부풀림) 현상을 방지하기 위해 80%에서 충전을 중단합니다.",
        )
    }

    pub fn battery_specs_line(&self, model: &str, cycles: u32, health: f32) -> String {
        let fmt = tr(*self,
            "Modelo Celda: {} • Ciclos: {} • Salud Química: {:.0}%",
            "Cell Model: {} • Cycles: {} • Chemistry Health: {:.0}%",
            "Zellmodell: {} • Zyklen: {} • Akkuzustand: {:.0}%",
            "Modèle cellule : {} • Cycles : {} • Santé chimique : {:.0}%",
            "Modelo Célula: {} • Ciclos: {} • Saúde Química: {:.0}%",
            "Modello cella: {} • Cicli: {} • Salute chimica: {:.0}%",
            "Модель ячейки: {} • Циклы: {} • Здоровье батареи: {:.0}%",
            "电芯型号: {} • 循环充放次数: {} • 化学健康度: {:.0}%",
            "セル形式: {} • 充放電回数: {} • バッテリー健全度: {:.0}%",
            "셀 모델: {} • 충방전 사이클: {} • 화학적 수명: {:.0}%",
        );
        let s = fmt.replacen("{}", model, 1);
        let s = s.replacen("{}", &cycles.to_string(), 1);
        s.replace("{:.0}", &format!("{:.0}", health))
    }

    pub fn gaming_locks_header(&self) -> &'static str {
        tr(*self,
            "🎮 Bloqueos Tácticos para Gaming y Pantalla",
            "🎮 Tactical Gaming Locks & Hardware Features",
            "🎮 Taktische Gaming-Sperren & Hardware-Features",
            "🎮 Verrouillages tactiques pour le jeu et écran",
            "🎮 Bloqueios Táticos para Jogos e Tela",
            "🎮 Blocchi tattici per gaming e display",
            "🎮 Тактические игровые блокировки и дисплей",
            "🎮 竞技外设防误触锁定与电竞屏幕超频",
            "🎮 ゲーミング誤爆防止ロック＆液晶機能",
            "🎮 택티컬 게이밍 락 및 디스플레이 오버드라이브",
        )
    }

    pub fn winkey_lock_label(&self) -> &'static str {
        tr(*self,
            "🔒 Bloquear tecla Windows / Super durante Gaming",
            "🔒 Lock Windows / Super Key during Gaming",
            "🔒 Windows- / Super-Taste im Spiel sperren",
            "🔒 Verrouiller la touche Windows / Super en jeu",
            "🔒 Bloquear tecla Windows / Super durante Jogos",
            "🔒 Blocca tasto Windows / Super durante il gioco",
            "🔒 Блокировать клавишу Windows / Super в играх",
            "🔒 锁定 Windows / Super 徽标键 (防止游戏中误触弹窗切屏)",
            "🔒 ゲーム中の Windows / Super キーを無効化",
            "🔒 게임 중 Windows / Super 키 잠금",
        )
    }

    pub fn winkey_lock_desc(&self) -> &'static str {
        tr(*self,
            "Desactiva el scancode de la tecla Super para evitar caídas accidentales al escritorio durante operaciones en pantalla completa.",
            "Disables the Super key scancode to avoid desktop dropouts during full-screen operations.",
            "Deaktiviert den Scancode der Super-Taste, um versehentliche Desktop-Abstürze im Vollbildmodus zu verhindern.",
            "Désactive le code de la touche Super pour éviter les retours intempestifs sur le bureau en plein écran.",
            "Desativa o scancode da tecla Super para evitar quedas acidentais na área de trabalho em tela cheia.",
            "Disattiva lo scancode del tasto Super per evitare ritorni accidentali al desktop in modalità a schermo intero.",
            "Отключает скан-код клавиши Super, предотвращая случайный вылет на рабочий стол в полноэкранных играх.",
            "在底层过滤 Super 键硬件扫描码，彻底根绝在激战全屏操作中误击弹回桌面崩溃的痛点。",
            "フルスクリーンでのゲームプレイ中に誤ってデスクトップに戻るのを防ぐため、キー入力を抑制します。",
            "전체화면 게임 플레이 중 실수로 바탕화면으로 튕기는 현상을 방지하기 위해 Super 키 입력을 차단합니다.",
        )
    }

    pub fn touchpad_lock_label(&self) -> &'static str {
        tr(*self,
            "🚫 Desactivar Touchpad Interno",
            "🚫 Disable Internal Touchpad",
            "🚫 Internes Touchpad deaktivieren",
            "🚫 Désactiver le pavé tactile interne",
            "🚫 Desativar Touchpad Interno",
            "🚫 Disattiva touchpad interno",
            "🚫 Отключить встроенный тачпад",
            "🚫 屏蔽笔记本内置触控板 (接驳外接鼠标时使用)",
            "🚫 内蔵タッチパッドを無効化",
            "🚫 내장 터치패드 비활성화",
        )
    }

    pub fn touchpad_lock_desc(&self) -> &'static str {
        tr(*self,
            "Suprime las interrupciones hardware del touchpad para eliminar toques accidentales de palma al usar ratón externo.",
            "Suppresses touchpad hardware interrupts to eliminate accidental palm touches when using an external mouse.",
            "Unterdrückt Touchpad-Hardware-Interrupts, um versehentliche Berührungen mit dem Handballen zu eliminieren.",
            "Supprime les interruptions matérielles du pavé tactile pour éliminer les effleurements accidentels.",
            "Suprime as interrupções de hardware do touchpad para eliminar toques acidentais com a palma da mão.",
            "Elimina i tocchi accidentali con il palmo disattivando gli interrupt hardware del touchpad.",
            "Подавляет аппаратные прерывания тачпада, исключая случайные задевания ладонью при игре с мышью.",
            "直接切断 I2C/PS2 硬件中断信号，杜绝使用外接电竞鼠标操作键盘时手掌误碰光标位移。",
            "外付けマウス使用時に手のひらによる誤操作を防ぐため、ハードウェア割り込みを停止します。",
            "외장 마우스 사용 시 손바닥 접촉으로 인한 오작동을 없애기 위해 하드웨어 인터럽트를 차단합니다.",
        )
    }

    pub fn lcd_overdrive_button(&self) -> &'static str {
        tr(*self,
            "🚀 Activar Overdrive de Respuesta LCD 3ms",
            "🚀 Trigger LCD 3ms Response Overdrive",
            "🚀 LCD 3ms Reaktions-Overdrive aktivieren",
            "🚀 Activer l'Overdrive LCD 3ms",
            "🚀 Ativar Overdrive de Resposta LCD 3ms",
            "🚀 Attiva LCD Overdrive 3ms",
            "🚀 Включить овердрайв отклика матрицы 3мс",
            "🚀 开启电竞显示面板 3ms 灰阶急速响应超频 (LCD Overdrive)",
            "🚀 LCD 3ms 応答速度オーバードライブ起動",
            "🚀 LCD 3ms 응답속도 오버드라이브 활성화",
        )
    }

    pub fn lcd_overdrive_desc(&self) -> &'static str {
        tr(*self,
            "Aplica sobrevoltaje acelerado al panel para eliminar imágenes fantasma (ghosting) en videojuegos con altas tasas de refresco.",
            "Applies accelerated voltage overdrive to the panel to eliminate ghosting in high-FPS gaming.",
            "Gibt Spannungsimpulse an das Panel ab, um Ghosting bei hohen Bildwiederholraten zu eliminieren.",
            "Applique une surtension accélérée à la dalle pour éliminer le ghosting à haute fréquence.",
            "Aplica sobretensão acelerada ao painel para eliminar fantasmas em jogos com altas taxas de atualização.",
            "Applica un overdrive di tensione per eliminare le scie (ghosting) nei titoli ad alto framerate.",
            "Подает повышенное напряжение на жидкие кристаллы, устраняя шлейфы (ghosting) в динамичных сценах.",
            "向液晶分子施加微秒级驱动加压，极限消除高帧率疾速竞技画面中的动态残影拖尾 (Ghosting)。",
            "液晶パネルの駆動電圧を最適化し、高フレームレート時の残像（ゴースト）を最小限に抑えます。",
            "패널에 가속 전압을 인가하여 고주사율 게이밍 시 발생하는 잔상(Ghosting) 현상을 제거합니다.",
        )
    }

    pub fn diagnostics_header(&self) -> &'static str {
        tr(*self,
            "🐧 Diagnóstico de Buses de Hardware y Kernel Linux",
            "🐧 Linux Kernel & Hardware Bus Diagnostics",
            "🐧 Linux-Kernel & Hardwarebus-Diagnose",
            "🐧 Diagnostics du noyau Linux et des bus matériels",
            "🐧 Diagnóstico de Barramentos de Hardware e Kernel Linux",
            "🐧 Diagnostica bus hardware e kernel Linux",
            "🐧 Диагностика шин оборудования и ядра Linux",
            "🐧 Linux 底层内核总线通信与控制器自检诊断",
            "🐧 Linux カーネル＆ハードウェアバス自己診断",
            "🐧 Linux 커널 및 하드웨어 버스 진단",
        )
    }

    pub fn run_diagnostics_btn(&self) -> &'static str {
        tr(*self,
            "🩺 Ejecutar Auto-Prueba de Hardware",
            "🩺 Run Hardware Self-Test",
            "🩺 Hardware-Selbsttest starten",
            "🩺 Lancer l'auto-test matériel",
            "🩺 Executar Autoteste de Hardware",
            "🩺 Esegui autotest hardware",
            "🩺 Запустить аппаратный тест",
            "🩺 执行底层硬件自检",
            "🩺 ハードウェア診断実行",
            "🩺 하드웨어 자체 진단 실행",
        )
    }

    pub fn diagnostics_result(&self, acpi: &str, nvme: &str, bat: &str, gpu: &str) -> String {
        let fmt = tr(*self,
            "Resultado Diagnóstico: Bus ACPI: {}, Controlador NVMe: {}, Sensor Batería: {}, Enlace NVIDIA: {}",
            "Diagnostic Result: ACPI Bus: {}, NVMe Controller: {}, Battery Sensor: {}, NVIDIA Bus: {}",
            "Diagnose-Ergebnis: ACPI-Bus: {}, NVMe-Controller: {}, Akku-Sensor: {}, NVIDIA-Bus: {}",
            "Résultat diagnostic : Bus ACPI : {}, Contrôleur NVMe : {}, Capteur batterie : {}, Bus NVIDIA : {}",
            "Resultado Diagnóstico: Barramento ACPI: {}, Controlador NVMe: {}, Sensor da Bateria: {}, Barramento NVIDIA: {}",
            "Risultato diagnosi: Bus ACPI: {}, Controller NVMe: {}, Sensore batteria: {}, Bus NVIDIA: {}",
            "Результат теста: Шина ACPI: {}, Контроллер NVMe: {}, Датчик батареи: {}, Шина NVIDIA: {}",
            "自检报告结果: ACPI 驱动总线: {} | NVMe 控制器: {} | 电池库传感器: {} | NVIDIA 链路: {}",
            "診断結果: ACPI バス: {} | NVMe コントローラ: {} | バッテリーセンサ: {} | NVIDIA リンク: {}",
            "진단 결과: ACPI 버스: {} | NVMe 컨트롤러: {} | 배터리 센서: {} | NVIDIA 버스: {}",
        );
        let s = fmt.replacen("{}", acpi, 1);
        let s = s.replacen("{}", nvme, 1);
        let s = s.replacen("{}", bat, 1);
        s.replace("{}", gpu)
    }

    // --- ABOUT & GITHUB COMMUNITY ---
    pub fn about_github_header(&self) -> &'static str {
        tr(*self,
            "5. Proyecto AcerSense & Comunidad GitHub",
            "5. AcerSense Project & GitHub Community",
            "5. AcerSense-Projekt & GitHub-Community",
            "5. Projet AcerSense et communauté GitHub",
            "5. Projeto AcerSense e Comunidade GitHub",
            "5. Progetto AcerSense e comunità GitHub",
            "5. Проект AcerSense и сообщество GitHub",
            "5. AcerSense 开源项目与 GitHub 开发者社区",
            "5. AcerSense プロジェクト＆ GitHub コミュニティ",
            "5. AcerSense 프로젝트 및 GitHub 개발자 커뮤니티",
        )
    }

    pub fn about_github_subtitle(&self) -> &'static str {
        tr(*self,
            "Suite libre de control de hardware para Linux. Visita el repositorio para código fuente, actualizaciones y reporte de errores.",
            "Open-source Linux hardware control suite. Visit the repository for source code, releases, and issue tracking.",
            "Open-Source-Hardwaresteuerung für Linux. Quellcode, Releases und Fehlerberichte auf GitHub.",
            "Suite open-source de contrôle matériel pour Linux. Code source, versions et signalement de bugs sur GitHub.",
            "Suite de controle de hardware de código aberto para Linux. Código-fonte, versões e rastreador de bugs no GitHub.",
            "Suite open source per il controllo hardware su Linux. Codice sorgente, rilasci e segnalazione bug su GitHub.",
            "Пакет управления оборудованием Linux с открытым исходным кодом. Исходники, релизы и баг-трекер на GitHub.",
            "专为 Linux 打造的开源硬件控制套件。欢迎访问 GitHub 仓库获取源码、发布包并提交 Issue 反馈。",
            "Linux向けオープンソース・ハードウェア制御スイート。ソースコード、リリース、Issue報告はこちら。",
            "Linux용 오픈 소스 하드웨어 제어 제품군. 소스 코드, 릴리스 및 버그 보고는 GitHub를 방문하세요.",
        )
    }

    pub fn btn_project_repo(&self) -> &'static str {
        tr(*self,
            "🐙 Repositorio del Proyecto",
            "🐙 Project Repository",
            "🐙 Projekt-Repository",
            "🐙 Dépôt du projet",
            "🐙 Repositório do Projeto",
            "🐙 Repository del progetto",
            "🐙 Репозиторий проекта",
            "🐙 项目官方仓库 (GitHub)",
            "🐙 プロジェクト リポジトリ",
            "🐙 프로젝트 저장소",
        )
    }

    pub fn btn_author_profile(&self) -> &'static str {
        tr(*self,
            "👤 Perfil del Autor (@rodrigo47363)",
            "👤 Author Profile (@rodrigo47363)",
            "👤 Entwicklerprofil (@rodrigo47363)",
            "👤 Profil de l'auteur (@rodrigo47363)",
            "👤 Perfil do Autor (@rodrigo47363)",
            "👤 Profilo dell'autore (@rodrigo47363)",
            "👤 Профиль автора (@rodrigo47363)",
            "👤 开发者主页 (@rodrigo47363)",
            "👤 開発者プロフィール (@rodrigo47363)",
            "👤 개발자 프로필 (@rodrigo47363)",
        )
    }

    pub fn btn_report_issue(&self) -> &'static str {
        tr(*self,
            "🐞 Reportar Fallo / Issues",
            "🐞 Report Issue / Bug Tracker",
            "🐞 Fehler melden / Issues",
            "🐞 Signaler un bug / Issues",
            "🐞 Relatar Problema / Issues",
            "🐞 Segnala un problema / Issues",
            "🐞 Сообщить об ошибке / Issues",
            "🐞 反馈 Bug / 提交 Issues",
            "🐞 不具合報告 / Issues",
            "🐞 이슈 제보 / 버그 트래커",
        )
    }

    pub fn btn_releases(&self) -> &'static str {
        tr(*self,
            "⭐ Releases & Actualizaciones",
            "⭐ Releases & Changelog",
            "⭐ Releases & Aktualisierungen",
            "⭐ Versions et nouveautés",
            "⭐ Versões e Novidades",
            "⭐ Versioni e Changelog",
            "⭐ Релизы и обновления",
            "⭐ 发行版本与更新日志",
            "⭐ リリース＆更新履歴",
            "⭐ 릴리스 및 업데이트 내역",
        )
    }

    pub fn about_meta_info(&self) -> &'static str {
        tr(*self,
            "Versión: v2.1.0 (Rust Edition) • Licencia: GPL-3.0 • Desarrollado por Rodrigo",
            "Version: v2.1.0 (Rust Edition) • License: GPL-3.0 • Developed by Rodrigo",
            "Version: v2.1.0 (Rust Edition) • Lizenz: GPL-3.0 • Entwickelt von Rodrigo",
            "Version : v2.1.0 (Rust Edition) • Licence : GPL-3.0 • Développé par Rodrigo",
            "Versão: v2.1.0 (Rust Edition) • Licença: GPL-3.0 • Desenvolvido por Rodrigo",
            "Versione: v2.1.0 (Rust Edition) • Licenza: GPL-3.0 • Sviluppato da Rodrigo",
            "Версия: v2.1.0 (Rust Edition) • Лицензия: GPL-3.0 • Разработчик: Rodrigo",
            "版本: v2.1.0 (Rust Edition) • 许可证: GPL-3.0 • 开发者: Rodrigo",
            "バージョン: v2.1.0 (Rust Edition) • ライセンス: GPL-3.0 • 開発者: Rodrigo",
            "버전: v2.1.0 (Rust Edition) • 라이선스: GPL-3.0 • 개발자: Rodrigo",
        )
    }

    // --- OSCILLOSCOPE & MONITORING HELPERS ---
    pub fn purge_turbo_btn(&self) -> &'static str {
        tr(*self,
            "PURGAR TURBO", "PURGE TURBO", "TURBO LÖSCHEN", "PURGER TURBO", "PURGAR TURBO",
            "PURGA TURBO", "СБРОС ТУРБО", "重置狂暴模式", "ターボ解除", "터보 해제",
        )
    }

    pub fn reset_auto_btn(&self) -> &'static str {
        tr(*self,
            "REINICIO AUTO", "RESET AUTO", "AUTO RESET", "RÉINIT AUTO", "REINÍCIO AUTO",
            "RESET AUTO", "АВТО-СБРОС", "恢复自动调度", "自動リセット", "자동 복구",
        )
    }

    pub fn oscilloscope_title(&self) -> &'static str {
        tr(*self,
            "📊 Osciloscopio de Hardware en Vivo",
            "📊 Live Hardware Oscilloscope",
            "📊 Live-Hardware-Oszilloskop",
            "📊 Oscilloscope matériel en direct",
            "📊 Osciloscópio de Hardware em Tempo Real",
            "📊 Oscilloscopio hardware dal vivo",
            "📊 Аппаратный осциллограф реального времени",
            "📊 60 FPS 硬件高频动态波形示波器",
            "📊 リアルタイム ハードウェア オシロスコープ",
            "📊 실시간 하드웨어 오실로스코프",
        )
    }

    pub fn window_label(&self) -> &'static str {
        tr(*self,
            "Ventana:", "Window:", "Zeitfenster:", "Fenêtre :", "Janela:",
            "Finestra:", "Окно:", "观察时间窗:", "時間窓:", "시간 창:",
        )
    }

    pub fn status_frozen(&self) -> &'static str {
        tr(*self,
            "⏸ PAUSADO", "⏸ FROZEN", "⏸ ANGEHALTEN", "⏸ FIGÉ", "⏸ PAUSADO",
            "⏸ SOSPESO", "⏸ ПАУЗА", "⏸ 已冻结捕获", "⏸ 停止中", "⏸ 일시정지됨",
        )
    }

    pub fn status_live(&self) -> &'static str {
        tr(*self,
            "🟢 EN VIVO 60 FPS", "🟢 LIVE 60 FPS", "🟢 LIVE 60 FPS", "🟢 EN DIRECT 60 FPS", "🟢 EM TEMPO REAL 60 FPS",
            "🟢 IN DIRETTA 60 FPS", "🟢 ПРЯМОЙ ЭФИР 60 FPS", "🟢 60 FPS 极速采样中", "🟢 リアルタイム 60 FPS", "🟢 실시간 60 FPS",
        )
    }

    pub fn legend_cpu_package(&self) -> &'static str {
        tr(*self,
            "● Paquete CPU:", "● CPU Package:", "● CPU-Paket:", "● Package CPU :", "● Pacote CPU:",
            "● Pacchetto CPU:", "● ЦП (CPU):", "● CPU 封装核心:", "● CPU パッケージ:", "● CPU 패키지:",
        )
    }

    pub fn legend_discrete_gpu(&self) -> &'static str {
        tr(*self,
            "● GPU Dedicada:", "● Discrete GPU:", "● Dedizierte GPU:", "● GPU Dédiée :", "● GPU Dedicada:",
            "● GPU Dedicata:", "● ГП (GPU):", "● 独立 GPU 核心:", "● 独立 GPU:", "● 외장 GPU:",
        )
    }

    pub fn legend_nvme(&self) -> &'static str {
        tr(*self,
            "● SSD NVMe:", "● NVMe SSD:", "● NVMe-SSD:", "● SSD NVMe :", "● SSD NVMe:",
            "● SSD NVMe:", "● NVMe SSD:", "● NVMe 固态主控:", "● NVMe SSD:", "● NVMe SSD:",
        )
    }

    pub fn legend_cpu_load(&self) -> &'static str {
        tr(*self,
            "● Carga CPU:", "● CPU Load:", "● CPU-Last:", "● Charge CPU :", "● Carga CPU:",
            "● Carico CPU:", "● Загрузка CPU:", "● CPU 综合负载:", "● CPU 負荷:", "● CPU 부하:",
        )
    }

    pub fn legend_gpu_load(&self) -> &'static str {
        tr(*self,
            "● Carga Núcleo GPU:", "● GPU Core Load:", "● GPU-Kernlast:", "● Charge cœur GPU :", "● Carga Núcleo GPU:",
            "● Carico core GPU:", "● Загрузка ядра GPU:", "● GPU 渲染核心负载:", "● GPU コア負荷:", "● GPU 코어 부하:",
        )
    }

    pub fn legend_combined(&self) -> &'static str {
        tr(*self,
            "● Combinada:", "● Combined:", "● Kombiniert:", "● Combinée :", "● Combinada:",
            "● Combinata:", "● Суммарная:", "● 综合功耗:", "● 合計:", "● 통합:",
        )
    }

    pub fn legend_cpu_fan(&self) -> &'static str {
        tr(*self,
            "● Ventilador CPU:", "● CPU Fan:", "● CPU-Lüfter:", "● Ventilateur CPU :", "● Ventoinha CPU:",
            "● Ventola CPU:", "● Куллер CPU:", "● CPU 涡轮转速:", "● CPU ファン:", "● CPU 팬:",
        )
    }

    pub fn legend_gpu_fan(&self) -> &'static str {
        tr(*self,
            "● Ventilador GPU:", "● GPU Fan:", "● GPU-Lüfter:", "● Ventilateur GPU :", "● Ventoinha GPU:",
            "● Ventola GPU:", "● Куллер GPU:", "● GPU 涡轮转速:", "● GPU ファン:", "● GPU 팬:",
        )
    }

    // --- HARDWARE COMPONENT CARDS ---
    pub fn cpu_arch_title(&self) -> &'static str {
        tr(*self,
            "🖥 Arquitectura de Silicio CPU",
            "🖥 CPU Silicon Architecture",
            "🖥 CPU-Silizium-Architektur",
            "🖥 Architecture silicium CPU",
            "🖥 Arquitetura de Silício da CPU",
            "🖥 Architettura silicio CPU",
            "🖥 Кремниевая архитектура CPU",
            "🖥 CPU 核心架构与主频拓扑",
            "🖥 CPU シリコン アーキテクチャ",
            "🖥 CPU 실리콘 아키텍처",
        )
    }

    pub fn instant_load_label(&self) -> &'static str {
        tr(*self,
            "Carga Instantánea:", "Instant Load:", "Momentanlast:", "Charge instantanée :", "Carga Instantânea:",
            "Carico istantaneo:", "Тек. нагрузка:", "瞬时使用率:", "瞬間負荷:", "순간 부하율:",
        )
    }

    pub fn dynamic_boost_label(&self) -> &'static str {
        tr(*self,
            "Frecuencia Boost Dinámica:", "Dynamic Boost Frequency:", "Dynamischer Boost-Takt:", "Fréquence Boost dynamique :", "Frequência Boost Dinâmica:",
            "Frequenza boost dinamica:", "Частота динамического буста:", "动态加速最高睿频:", "動的ブースト周波数:", "동적 부스트 주파수:",
        )
    }

    pub fn gpu_discrete_title(&self) -> &'static str {
        tr(*self,
            "🎮 Gráficos Dedicados NVIDIA",
            "🎮 Discrete NVIDIA Graphics",
            "🎮 Dedizierte NVIDIA-Grafik",
            "🎮 Carte graphique dédiée NVIDIA",
            "🎮 Gráficos Dedicados NVIDIA",
            "🎮 Grafica dedicata NVIDIA",
            "🎮 Дискретная графика NVIDIA",
            "🎮 NVIDIA 独立高性能图形芯片",
            "🎮 ディスクリート NVIDIA グラフィックス",
            "🎮 외장 NVIDIA 그래픽스",
        )
    }

    pub fn gpu_standby_badge(&self) -> &'static str {
        tr(*self,
            "PCIe D3cold EN ESPERA", "PCIe D3cold STANDBY", "PCIe D3cold STANDBY", "PCIe D3cold EN VEILLE", "PCIe D3cold EM ESPERA",
            "PCIe D3cold IN STANDBY", "PCIe D3cold ОЖИДАНИЕ", "PCIe D3cold 深度节能休眠", "PCIe D3cold スタンバイ", "PCIe D3cold 대기 모드",
        )
    }

    pub fn silicon_utilization(&self) -> &'static str {
        tr(*self,
            "Uso de Silicio:", "Silicon Utilization:", "Chipaustastung:", "Utilisation du silicium :", "Uso de Silício:",
            "Utilizzo silicio:", "Использование чипа:", "核心硬件利用率:", "コア使用率:", "실리콘 사용률:",
        )
    }

    pub fn dedicated_vram(&self) -> &'static str {
        tr(*self,
            "VRAM Dedicada:", "Dedicated VRAM:", "Dedizierter VRAM:", "VRAM Dédiée :", "VRAM Dedicada:",
            "VRAM Dedicata:", "Выделенная VRAM:", "独立显存占用:", "専用 VRAM:", "전용 VRAM:",
        )
    }

    pub fn gpu_d3cold_desc(&self) -> (&'static str, &'static str) {
        tr2(*self,
            ("Estado PCIe D3cold de consumo ultrabajo (0.0 W)", "La turbina GPU está en modo silencioso 0 RPM para conservar energía y eliminar el ruido."),
            ("PCIe D3cold Ultra-Low Power State (0.0 W)", "GPU turbine is in Zero-RPM silent mode to conserve energy and eliminate noise."),
            ("PCIe D3cold Ultraniedrigenergie (0,0 W)", "GPU-Lüfter ist im Zero-RPM-Modus, um Strom zu sparen und Lärm zu vermeiden."),
            ("État PCIe D3cold ultra-basse consommation (0,0 W)", "Le ventilateur GPU est à 0 tr/min pour préserver l'énergie."),
            ("Estado PCIe D3cold de consumo ultrabaixo (0,0 W)", "A turbina da GPU está em modo 0 RPM para poupar energia e eliminar ruído."),
            ("Stato PCIe D3cold ultra-basso consumo (0.0 W)", "La turbina GPU è in modalità 0 RPM per risparmiare energia."),
            ("Состояние ультранизкого питания PCIe D3cold (0.0 W)", "Кулер GPU отключен (0 RPM) для экономии энергии и тишины."),
            ("PCIe D3cold 极低功耗待机状态 (0.0 W)", "独显风扇保持 0 RPM 停转静音状态，消除噪音并大幅度节约整机功耗。"),
            ("PCIe D3cold 超省電力状態 (0.0 W)", "省電力と静音化のため、GPUファンは 0 RPM 停止モードに入っています。"),
            ("PCIe D3cold 초저전력 대기 상태 (0.0 W)", "GPU 터빈이 0 RPM 무소음 모드로 진입하여 전력을 절약하고 소음을 제거합니다."),
        )
    }

    pub fn memory_subsystem_title(&self) -> &'static str {
        tr(*self,
            "💾 Subsistema de Memoria", "💾 Memory Subsystem", "💾 Speicher-Subsystem", "💾 Sous-système mémoire", "💾 Subsistema de Memória",
            "💾 Sottosistema memoria", "💾 Подсистема памяти", "💾 物理与虚拟内存子系统", "💾 メモリー サブシステム", "💾 메모리 서브시스템",
        )
    }

    pub fn physical_ram(&self) -> &'static str {
        tr(*self,
            "RAM Física:", "Physical RAM:", "Physischer RAM:", "RAM Physique :", "RAM Física:",
            "RAM Fisica:", "Физ. память:", "物理 RAM 内存:", "物理 RAM:", "물리 RAM:",
        )
    }

    pub fn virtual_swap(&self) -> &'static str {
        tr(*self,
            "Swap Virtual:", "Virtual Swap:", "Virtueller Swap:", "Swap Virtuel :", "Swap Virtual:",
            "Swap Virtuale:", "Вирт. Swap:", "虚拟 Swap 交换分区:", "仮想 Swap:", "가상 Swap:",
        )
    }

    pub fn nvme_storage_title(&self) -> &'static str {
        tr(*self,
            "💽 Unidad de Estado Sólido NVMe", "💽 NVMe Solid State Drive", "💽 NVMe-Solid-State-Laufwerk", "💽 Disque SSD NVMe", "💽 Unidade de Estado Sólido NVMe",
            "💽 Unità a stato solido NVMe", "💽 Твердотельный накопитель NVMe", "💽 NVMe 高速固态硬盘", "💽 NVMe ソリッドステート ドライブ", "💽 NVMe 솔리드 스테이트 드라이브",
        )
    }

    pub fn controller_state_label(&self) -> &'static str {
        tr(*self,
            "Estado del Controlador:", "Controller State:", "Controller-Status:", "État du contrôleur :", "Estado do Controlador:",
            "Stato controller:", "Состояние контроллера:", "固件主控状态:", "コントローラ状態:", "컨트롤러 상태:",
        )
    }

    pub fn controller_state_val(&self) -> &'static str {
        tr(*self,
            "En Línea / Óptimo", "Online / Optimal", "Online / Optimal", "En ligne / Optimal", "Online / Ideal",
            "Online / Ottimale", "В сети / Норма", "在线 / 最佳工况", "稼働中 / 正常", "온라인 / 최적 상태",
        )
    }

    pub fn life_integrity(&self, pct: u8) -> String {
        let fmt = tr(*self,
            "Integridad de Vida: {}%", "Life Integrity: {}%", "Lebensdauer-Integrität: {}%", "Intégrité de vie : {}%", "Integridade de Vida: {}%",
            "Integrità operativa: {}%", "Остаточный ресурс: {}%", "颗粒剩余寿命健康度: {}%", "寿命健全性: {}%", "수명 잔여 무결성: {}%",
        );
        fmt.replace("{}", &pct.to_string())
    }

    pub fn power_delivery_title(&self) -> &'static str {
        tr(*self,
            "⚡ Alimentación y Batería", "⚡ Power Delivery & Battery", "⚡ Stromversorgung & Akku", "⚡ Alimentation et batterie", "⚡ Alimentação e Bateria",
            "⚡ Alimentazione e batteria", "⚡ Питание и аккумулятор", "⚡ 电源供电拓扑与电池", "⚡ 電源供給＆バッテリー", "⚡ 전원 공급 및 배터리",
        )
    }

    pub fn aeroblade_turbines_title(&self) -> &'static str {
        tr(*self,
            "🌀 Turbinas AeroBlade", "🌀 AeroBlade Turbines", "🌀 AeroBlade-Turbinen", "🌀 Turbines AeroBlade", "🌀 Turbinas AeroBlade",
            "🌀 Turbine AeroBlade", "🌀 Турбины AeroBlade", "🌀 AeroBlade 3D 金属涡轮", "🌀 AeroBlade タービン", "🌀 AeroBlade 터빈 팬",
        )
    }

    pub fn cpu_turbine_label(&self) -> &'static str {
        tr(*self,
            "Turbina CPU:", "CPU Turbine:", "CPU-Turbine:", "Turbine CPU :", "Turbina CPU:",
            "Turbina CPU:", "Турбина CPU:", "CPU 散热涡轮:", "CPU タービン:", "CPU 터빈:",
        )
    }

    pub fn gpu_turbine_label(&self) -> &'static str {
        tr(*self,
            "Turbina GPU:", "GPU Turbine:", "GPU-Turbine:", "Turbine GPU :", "Turbina GPU:",
            "Turbina GPU:", "Турбина GPU:", "GPU 散热涡轮:", "GPU タービン:", "GPU 터빈:",
        )
    }

    // --- SCENARIOS (POWER MODES) HELPERS ---
    pub fn energy_governance_title(&self) -> &'static str {
        tr(*self,
            "🔌 Gobernanza Energética y Estado de Potencia Linux",
            "🔌 Energy Governance & Linux Power State",
            "🔌 Energieverwaltung & Linux-Power-Status",
            "🔌 Gestion de l'énergie et état d'alimentation Linux",
            "🔌 Governança Energética e Estado de Energia Linux",
            "🔌 Governance energetica e stato alimentazione Linux",
            "🔌 Управление энергопотреблением в ядре Linux",
            "🔌 Linux 内核能效调度器与硬件电源状态 (EPP)",
            "🔌 Linux エネルギー ガバナンス＆電源管理",
            "🔌 Linux 에너지 거버넌스 및 전원 관리 상태",
        )
    }

    pub fn realtime_dissipation_label(&self) -> &'static str {
        tr(*self,
            "Disipación en Tiempo Real:", "Real-Time Dissipation:", "Echtzeit-Verlustleistung:", "Dissipation en temps réel :", "Dissipação em Tempo Real:",
            "Dissipazione in tempo reale:", "Тепловыделение:", "实时硬件总功耗放热:", "リアルタイム消費電力放熱:", "실시간 소비 전력 방열:",
        )
    }

    pub fn active_governor_label(&self, gov: &str, epp: &str) -> String {
        let fmt = tr(*self,
            "Gobernador Activo: {} | EPP: {}",
            "Active Governor: {} | EPP: {}",
            "Aktiver Governor: {} | EPP: {}",
            "Gouverneur actif : {} | EPP : {}",
            "Governador Ativo: {} | EPP: {}",
            "Governor attivo: {} | EPP: {}",
            "Активный регулятор: {} | EPP: {}",
            "当前内核调频策略: {} | 能量性能偏好 (EPP): {}",
            "現在のガバナー: {} | EPP: {}",
            "활성 거버너: {} | EPP: {}",
        );
        let s = fmt.replacen("{}", gov, 1);
        s.replace("{}", epp)
    }

    // --- KEYBOARD RGB HELPERS ---
    pub fn quick_palette_title(&self) -> &'static str {
        tr(*self,
            "Paleta Táctica Rápida:", "Quick Tactical Palette:", "Taktische Schnellfarben:", "Palette tactique rapide :", "Paleta Tática Rápida:",
            "Palette tattica rapida:", "Быстрая тактическая палитра:", "电竞战术快捷色板:", "クイック タクティカル パレット:", "빠른 택티컬 팔레트:",
        )
    }

    pub fn custom_mixer_title(&self) -> &'static str {
        tr(*self,
            "Mezclador de Color Personalizado:", "Custom Color Mixer:", "Individueller Farbmischer:", "Mélangeur de couleurs personnalisé :", "Misturador de Cores Personalizado:",
            "Mixer colore personalizzato:", "Пользовательский микшер цвета:", "自定义精准 RGB 混色器:", "カスタム カラー ミキサー:", "사용자 지정 컬러 믹서:",
        )
    }

    pub fn push_color_btn(&self) -> &'static str {
        tr(*self,
            "⚡ Enviar Color a Zona Seleccionada",
            "⚡ Push Color to Selected Zone",
            "⚡ Farbe an gewählte Zone senden",
            "⚡ Envoyer la couleur à la zone",
            "⚡ Enviar Cor para a Zona Selecionada",
            "⚡ Invia colore alla zona selezionata",
            "⚡ Применить цвет к выбранной зоне",
            "⚡ 写入颜色至当前选定分区",
            "⚡ 選択したゾーンへ色を適用",
            "⚡ 선택한 구역에 색상 전송",
        )
    }

    pub fn master_presets_title(&self) -> &'static str {
        tr(*self,
            "Presets Multizona Acer Nitro:", "Acer Nitro Multi-Zone Presets:", "Acer Nitro Mehrzonen-Profile:", "Préréglages multizones Acer Nitro :", "Predefinições Multizona Acer Nitro:",
            "Preset multizona Acer Nitro:", "Многозонные пресеты Acer Nitro:", "Acer Nitro 出厂多区背光联动预设:", "Acer Nitro マルチゾーン プリセット:", "Acer Nitro 멀티 구역 프리셋:",
        )
    }

    pub fn applied_to_all_zones(&self, name: &str) -> String {
        let fmt = tr(*self,
            "Aplicado {} a Todas las Zonas",
            "Applied {} to All Zones",
            "{} auf alle Zonen angewendet",
            "{} appliqué à toutes les zones",
            "Aplicado {} a Todas as Zonas",
            "Applicato {} a tutte le zone",
            "Применено: {} ко всем зонам",
            "已将「{}」同步应用至所有背光分区",
            "「{}」を全ゾーンに適用しました",
            "모든 구역에 {} 적용 완료",
        );
        fmt.replace("{}", name)
    }

    pub fn applied_to_zone(&self, name: &str, zone: u8) -> String {
        let fmt = tr(*self,
            "Aplicado {} a Zona {}",
            "Applied {} to Zone {}",
            "{} auf Zone {} angewendet",
            "{} appliqué à la zone {}",
            "Aplicado {} à Zona {}",
            "Applicato {} alla zona {}",
            "Применено: {} к зоне {}",
            "已将「{}」写入背光分区 {}",
            "「{}」をゾーン {} に適用しました",
            "구역 {}에 {} 적용 완료",
        );
        let s = fmt.replacen("{}", name, 1);
        s.replace("{}", &zone.to_string())
    }

    pub fn pushed_to_all_zones(&self, r: u8, g: u8, b: u8) -> String {
        let hex = format!("#{:02X}{:02X}{:02X}", r, g, b);
        let fmt = tr(*self,
            "Enviado {} a Todas las Zonas",
            "Pushed {} to All Zones",
            "{} an alle Zonen gesendet",
            "{} envoyé à toutes les zones",
            "Enviado {} para Todas as Zonas",
            "Inviato {} a tutte le zone",
            "Отправлено: {} на все зоны",
            "已将色彩 {} 广播至全分区",
            "{} を全ゾーンへ送信しました",
            "모든 구역에 {} 전송 완료",
        );
        fmt.replace("{}", &hex)
    }

    pub fn pushed_to_zone(&self, r: u8, g: u8, b: u8, zone: u8) -> String {
        let hex = format!("#{:02X}{:02X}{:02X}", r, g, b);
        let fmt = tr(*self,
            "Enviado {} a Zona {}",
            "Pushed {} to Zone {}",
            "{} an Zone {} gesendet",
            "{} envoyé à la zone {}",
            "Enviado {} para Zona {}",
            "Inviato {} alla zona {}",
            "Отправлено: {} на зону {}",
            "已将色彩 {} 写入分区 {}",
            "{} をゾーン {} へ送信しました",
            "구역 {}에 {} 전송 완료",
        );
        let s = fmt.replacen("{}", &hex, 1);
        s.replace("{}", &zone.to_string())
    }

    pub fn applied_preset(&self, label: &str) -> String {
        let fmt = tr(*self,
            "Preset aplicado: {}",
            "Applied preset: {}",
            "Profil angewendet: {}",
            "Préréglage appliqué : {}",
            "Predefinição aplicada: {}",
            "Preset applicato: {}",
            "Применен пресет: {}",
            "动态光效预设已生效: {}",
            "プリセット適用完了: {}",
            "프리셋 적용 완료: {}",
        );
        fmt.replace("{}", label)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_language_parsing() {
        assert_eq!(Language::from_str("es"), Language::Es);
        assert_eq!(Language::from_str("spanish"), Language::Es);
        assert_eq!(Language::from_str("español"), Language::Es);
        assert_eq!(Language::from_str("en"), Language::En);
        assert_eq!(Language::from_str("english"), Language::En);
        assert_eq!(Language::from_str("de"), Language::De);
        assert_eq!(Language::from_str("deutsch"), Language::De);
        assert_eq!(Language::from_str("fr"), Language::Fr);
        assert_eq!(Language::from_str("français"), Language::Fr);
        assert_eq!(Language::from_str("pt"), Language::Pt);
        assert_eq!(Language::from_str("português"), Language::Pt);
        assert_eq!(Language::from_str("it"), Language::It);
        assert_eq!(Language::from_str("italiano"), Language::It);
        assert_eq!(Language::from_str("ru"), Language::Ru);
        assert_eq!(Language::from_str("русский"), Language::Ru);
        assert_eq!(Language::from_str("zh"), Language::Zh);
        assert_eq!(Language::from_str("简体中文"), Language::Zh);
        assert_eq!(Language::from_str("ja"), Language::Ja);
        assert_eq!(Language::from_str("日本語"), Language::Ja);
        assert_eq!(Language::from_str("ko"), Language::Ko);
        assert_eq!(Language::from_str("한국어"), Language::Ko);
        assert_eq!(Language::from_str("unknown"), Language::Es);
    }

    #[test]
    fn test_language_cycling() {
        let mut curr = Language::Es;
        for expected in [
            Language::En,
            Language::De,
            Language::Fr,
            Language::Pt,
            Language::It,
            Language::Ru,
            Language::Zh,
            Language::Ja,
            Language::Ko,
            Language::Es,
        ] {
            curr = curr.next();
            assert_eq!(curr, expected);
        }
    }

    #[test]
    fn test_translations_coverage() {
        for lang in Language::ALL {
            assert!(!lang.tab_fans().is_empty());
            assert!(!lang.tab_monitoring().is_empty());
            assert!(!lang.tab_power().is_empty());
            assert!(!lang.tab_rgb().is_empty());
            assert!(!lang.tab_settings().is_empty());
            assert!(!lang.fan_title().is_empty());
            assert!(!lang.footer_shortcuts().is_empty());
            assert!(!lang.settings_lang_header().is_empty());
            assert!(!lang.badge().is_empty());
            assert!(!lang.name().is_empty());
            assert!(!lang.about_github_header().is_empty());
            assert!(!lang.about_github_subtitle().is_empty());
            assert!(!lang.btn_project_repo().is_empty());
            assert!(!lang.btn_author_profile().is_empty());
            assert!(!lang.btn_report_issue().is_empty());
            assert!(!lang.btn_releases().is_empty());
            assert!(!lang.about_meta_info().is_empty());
        }
    }
}

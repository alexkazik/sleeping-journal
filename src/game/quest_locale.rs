use crate::game::data::NUM_QUESTS;
use crate::game::{GameLanguage, QuestId};
use std::array;
use std::iter::Copied;
use std::slice::Iter;

pub(crate) struct QuestLocale {
    language: GameLanguage,
    translation: [(QuestId, &'static str); NUM_QUESTS],
}

impl QuestLocale {
    #[must_use]
    pub(crate) fn new() -> Self {
        let language = GameLanguage::default();
        #[allow(clippy::cast_possible_truncation)]
        let mut result = Self {
            language,
            translation: array::from_fn(|i| (QuestId(i as u16), "")),
        };
        result.set_language(language);
        result
    }

    #[inline]
    pub(crate) fn all_str(&self) -> Copied<Iter<'_, (QuestId, &'static str)>> {
        self.translation.iter().copied()
    }

    #[inline]
    #[must_use]
    pub(crate) fn language(&self) -> GameLanguage {
        self.language
    }

    pub(crate) fn set_language(&mut self, language: GameLanguage) {
        self.language = language;
        for (q, n) in &mut self.translation {
            *n = Self::quest_name(language, *q);
        }
        self.translation.sort_by_key(|(_, n)| *n);
    }

    #[inline]
    #[must_use]
    pub(crate) fn get(&self, q: QuestId) -> &'static str {
        Self::quest_name(self.language, q)
    }

    #[must_use]
    pub(crate) fn try_get(&self, name: &str) -> Option<QuestId> {
        self.all_str().find(|(_, n)| *n == name).map(|(q, _)| q)
    }

    #[must_use]
    fn quest_name(language: GameLanguage, q: QuestId) -> &'static str {
        match q.0 {
            0 => language.card_1(),
            1 => language.card_2(),
            2 => language.card_13(),
            3 => language.card_4(),
            4 => language.card_125(),
            5 => language.card_107(),
            6 => language.card_144(),
            7 => language.card_199(),
            8 => language.card_57(),
            9 => language.card_146(),
            10 => language.card_19(),
            11 => language.card_166(),
            12 => language.card_124(),
            13 => language.card_37(),
            14 => language.card_158(),
            15 => language.card_196(),
            16 => language.card_117(),
            17 => language.card_208(),
            18 => language.card_89(),
            19 => language.card_218(),
            20 => language.card_193(),
            21 => language.card_160(),
            22 => language.card_143(),
            23 => language.card_105(),
            24 => language.card_163(),
            25 => language.card_15(),
            26 => language.card_216(),
            27 => language.card_54(),
            28 => language.card_119(),
            29 => language.card_151(),
            30 => language.card_23(),
            31 => language.card_93(),
            32 => language.card_112(),
            33 => language.card_154(),
            34 => language.card_9(),
            35 => language.card_215(),
            36 => language.card_198(),
            37 => language.card_85(),
            38 => language.card_28(),
            39 => language.card_138(),
            40 => language.card_49(),
            41 => language.card_213(),
            42 => language.card_131(),
            43 => language.card_48(),
            44 => language.card_17(),
            45 => language.card_145(),
            46 => language.card_43(),
            47 => language.card_202(),
            48 => language.card_102(),
            49 => language.card_187(),
            50 => language.card_184(),
            51 => language.card_77(),
            52 => language.card_194(),
            53 => language.card_8(),
            54 => language.card_66(),
            55 => language.card_82(),
            56 => language.card_6(),
            57 => language.card_73(),
            58 => language.card_159(),
            59 => language.card_86(),
            60 => language.card_148(),
            61 => language.card_14(),
            62 => language.card_44(),
            63 => language.card_69(),
            64 => language.card_210(),
            65 => language.card_5(),
            66 => language.card_161(),
            67 => language.card_185(),
            68 => language.card_88(),
            69 => language.card_157(),
            70 => language.card_126(),
            71 => language.card_167(),
            72 => language.card_53(),
            73 => language.card_32(),
            74 => language.card_40(),
            75 => language.card_41(),
            76 => language.card_62(),
            77 => language.card_34(),
            78 => language.card_27(),
            79 => language.card_94(),
            80 => language.card_72(),
            81 => language.card_70(),
            82 => language.card_12(),
            83 => language.card_52(),
            84 => language.card_87(),
            85 => language.card_191(),
            86 => language.card_135(),
            87 => language.card_217(),
            88 => language.card_84(),
            89 => language.card_71(),
            90 => language.card_137(),
            91 => language.card_11(),
            92 => language.card_50(),
            93 => language.card_186(),
            94 => language.card_83(),
            95 => language.card_116(),
            96 => language.card_203(),
            97 => language.card_181(),
            98 => language.card_129(),
            99 => language.card_114(),
            100 => language.card_109(),
            101 => language.card_164(),
            102 => language.card_101(),
            103 => language.card_128(),
            104 => language.card_97(),
            105 => language.card_200(),
            106 => language.card_96(),
            107 => language.card_130(),
            108 => language.card_162(),
            109 => language.card_98(),
            110 => language.card_214(),
            111 => language.card_25(),
            112 => language.card_36(),
            113 => language.card_103(),
            114 => language.card_182(),
            115 => language.card_91(),
            116 => language.card_68(),
            117 => language.card_46(),
            118 => language.card_150(),
            119 => language.card_47(),
            120 => language.card_21(),
            121 => language.card_42(),
            122 => language.card_75(),
            123 => language.card_45(),
            124 => language.card_104(),
            125 => language.card_209(),
            126 => language.card_121(),
            127 => language.card_120(),
            128 => language.card_205(),
            129 => language.card_118(),
            130 => language.card_92(),
            131 => language.card_212(),
            132 => language.card_211(),
            133 => language.card_18(),
            134 => language.card_99(),
            135 => language.card_10(),
            136 => language.card_60(),
            137 => language.card_56(),
            138 => language.card_155(),
            139 => language.card_106(),
            140 => language.card_195(),
            141 => language.card_132(),
            142 => language.card_141(),
            143 => language.card_122(),
            144 => language.card_67(),
            145 => language.card_147(),
            146 => language.card_123(),
            147 => language.card_139(),
            148 => language.card_51(),
            149 => language.card_78(),
            150 => language.card_149(),
            151 => language.card_74(),
            152 => language.card_113(),
            153 => language.card_81(),
            154 => language.card_63(),
            155 => language.card_165(),
            156 => language.card_90(),
            157 => language.card_110(),
            158 => language.card_64(),
            159 => language.card_24(),
            160 => language.card_197(),
            161 => language.card_134(),
            162 => language.card_206(),
            163 => language.card_140(),
            164 => language.card_22(),
            165 => language.card_108(),
            166 => language.card_133(),
            167 => language.card_80(),
            168 => language.card_16(),
            169 => language.card_100(),
            170 => language.card_30(),
            171 => language.card_26(),
            172 => language.card_111(),
            173 => language.card_76(),
            174 => language.card_136(),
            175 => language.card_153(),
            176 => language.card_29(),
            177 => language.card_204(),
            178 => language.card_38(),
            179 => language.card_59(),
            180 => language.card_152(),
            181 => language.card_3(),
            182 => language.card_33(),
            183 => language.card_39(),
            184 => language.card_188(),
            185 => language.card_156(),
            186 => language.card_20(),
            187 => language.card_142(),
            188 => language.card_127(),
            189 => language.card_79(),
            190 => language.card_55(),
            191 => language.card_95(),
            192 => language.card_65(),
            193 => language.card_61(),
            194 => language.card_31(),
            195 => language.card_7(),
            196 => language.card_189(),
            197 => language.card_183(),
            198 => language.card_207(),
            199 => language.card_190(),
            200 => language.card_58(),
            201 => language.card_115(),
            202 => language.card_192(),
            203 => language.card_35(),
            _ => unreachable!(),
        }
    }
}

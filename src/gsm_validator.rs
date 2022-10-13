pub struct GsmValidator {
    gsm_char_codes: Vec<u16>,
    gsme_char_codes: Vec<u16>,
    gsme_tr_char_codes: Vec<u16>,
    gsm_tr_char_codes: Vec<u16>,
    gsm_es_char_codes: Vec<u16>,
    gsme_es_char_codes: Vec<u16>,
    gsm_pt_char_codes: Vec<u16>,
    gsme_pt_char_codes: Vec<u16>,
}

impl GsmValidator {
    pub fn new() -> GsmValidator {
        GsmValidator {
            // '@£$¥èéùìòÇ\nØø\rÅåΔ_ΦΓΛΩΠΨΣΘΞÆæßÉ\x20!"#¤%&\'()*+,-./0123456789:;<=>?¡ABCDEFGHIJKLMNOPQRSTUVWXYZÄÖÑÜ§¿abcdefghijklmnopqrstuvwxyzäöñüà\f^{}\\[~]|€'
            gsm_char_codes: vec![
                10, 12, 13, 32, 33, 34, 35, 36, 37, 38, 39, 40, 41, 42, 43, 44, 45, 46, 47, 48, 49,
                50, 51, 52, 53, 54, 55, 56, 57, 58, 59, 60, 61, 62, 63, 64, 65, 66, 67, 68, 69, 70,
                71, 72, 73, 74, 75, 76, 77, 78, 79, 80, 81, 82, 83, 84, 85, 86, 87, 88, 89, 90, 91,
                92, 93, 94, 95, 97, 98, 99, 100, 101, 102, 103, 104, 105, 106, 107, 108, 109, 110,
                111, 112, 113, 114, 115, 116, 117, 118, 119, 120, 121, 122, 123, 124, 125, 126,
                161, 163, 164, 165, 167, 191, 196, 197, 198, 199, 201, 209, 214, 216, 220, 223,
                224, 228, 229, 230, 232, 233, 236, 241, 242, 246, 248, 249, 252, 915, 916, 920,
                923, 926, 928, 931, 934, 936, 937, 8364,
            ],
            // '\f|^€{}[~]\\'
            gsme_char_codes: vec![12, 91, 92, 93, 94, 123, 124, 125, 126, 8364],
            // '\f^{}\[~]|'
            gsme_tr_char_codes: vec![
                12, 91, 92, 93, 94, 123, 124, 125, 126, 286, 287, 304, 305, 350, 351, 8364,
            ],
            // '@£$¥€éùıòÇ\nĞğ\rÅåΔ_ΦΓΛΩΠΨΣΘΞŞşßÉ\x20!"#¤%&\'()*+,-./0123456789:;<=>?İABCDEFGHIJKLMNOPQRSTUVWXYZÄÖÑÜ§çabcdefghijklmnopqrstuvwxyzäöñüà\f^{}\[~]|'
            gsm_tr_char_codes: vec![
                10, 12, 13, 32, 33, 34, 35, 36, 37, 38, 39, 40, 41, 42, 43, 44, 45, 46, 47, 48, 49,
                50, 51, 52, 53, 54, 55, 56, 57, 58, 59, 60, 61, 62, 63, 64, 65, 66, 67, 68, 69, 70,
                71, 72, 73, 74, 75, 76, 77, 78, 79, 80, 81, 82, 83, 84, 85, 86, 87, 88, 89, 90, 91,
                92, 93, 94, 95, 97, 98, 99, 100, 101, 102, 103, 104, 105, 106, 107, 108, 109, 110,
                111, 112, 113, 114, 115, 116, 117, 118, 119, 120, 121, 122, 123, 124, 125, 126,
                163, 164, 165, 167, 196, 197, 199, 201, 209, 214, 220, 223, 224, 228, 229, 231,
                233, 241, 242, 246, 249, 252, 286, 287, 304, 305, 350, 351, 915, 916, 920, 923,
                926, 928, 931, 934, 936, 937, 8364,
            ],
            // '@£$¥èéùìòÇ\nØø\rÅåΔ_ΦΓΛΩΠΨΣΘΞÆæßÉ\x20!"#¤%&\'()*+,-./0123456789:;<=>?¡ABCDEFGHIJKLMNOPQRSTUVWXYZÄÖÑÜ§¿abcdefghijklmnopqrstuvwxyzäöñüàç\f^{}\\[~]|ÁÍÓÚá€íóú'
            gsm_es_char_codes: vec![
                10, 12, 13, 32, 33, 34, 35, 36, 37, 38, 39, 40, 41, 42, 43, 44, 45, 46, 47, 48, 49,
                50, 51, 52, 53, 54, 55, 56, 57, 58, 59, 60, 61, 62, 63, 64, 65, 66, 67, 68, 69, 70,
                71, 72, 73, 74, 75, 76, 77, 78, 79, 80, 81, 82, 83, 84, 85, 86, 87, 88, 89, 90, 91,
                92, 93, 94, 95, 97, 98, 99, 100, 101, 102, 103, 104, 105, 106, 107, 108, 109, 110,
                111, 112, 113, 114, 115, 116, 117, 118, 119, 120, 121, 122, 123, 124, 125, 126,
                161, 163, 164, 165, 167, 191, 193, 196, 197, 198, 199, 201, 205, 209, 211, 214,
                216, 218, 220, 223, 224, 225, 228, 229, 230, 231, 232, 233, 236, 237, 241, 242,
                243, 246, 248, 249, 250, 252, 915, 916, 920, 923, 926, 928, 931, 934, 936, 937,
                8364,
            ],
            // 'ç\f^{}\\[~]|ÁÍÓÚá€íóú'
            gsme_es_char_codes: vec![
                12, 91, 92, 93, 94, 123, 124, 125, 126, 193, 205, 211, 218, 225, 231, 237, 243,
                250, 8364,
            ],
            // '@£$¥êéúíóç\nÔô\rÁáΔ_ªÇÀ∞^\\€Ó|ÂâÊÉ\x20!"#º%&\'()*+,-./0123456789:;<=>?ÍABCDEFGHIJKLMNOPQRSTUVWXYZÃÕÚÜ§~abcdefghijklmnopqrstuvwxyzãõ`üà\fΦΓ^ΩΠΨΣΘ{}\\[~]|'
            gsm_pt_char_codes: vec![
                10, 12, 13, 32, 33, 34, 35, 36, 37, 38, 39, 40, 41, 42, 43, 44, 45, 46, 47, 48, 49,
                50, 51, 52, 53, 54, 55, 56, 57, 58, 59, 60, 61, 62, 63, 64, 65, 66, 67, 68, 69, 70,
                71, 72, 73, 74, 75, 76, 77, 78, 79, 80, 81, 82, 83, 84, 85, 86, 87, 88, 89, 90, 91,
                92, 93, 94, 95, 96, 97, 98, 99, 100, 101, 102, 103, 104, 105, 106, 107, 108, 109,
                110, 111, 112, 113, 114, 115, 116, 117, 118, 119, 120, 121, 122, 123, 124, 125,
                126, 163, 165, 167, 170, 186, 192, 193, 194, 195, 199, 201, 202, 205, 211, 212,
                213, 218, 220, 224, 225, 226, 227, 231, 233, 234, 237, 242, 243, 244, 245, 250,
                252, 915, 916, 920, 928, 931, 934, 936, 937, 8364, 8734,
            ],
            // '\fΦΓ^ΩΠΨΣΘ{}\\[~]|'
            gsme_pt_char_codes: vec![
                12, 91, 92, 93, 94, 123, 124, 125, 126, 193, 194, 195, 202, 205, 211, 212, 213,
                218, 225, 226, 227, 231, 234, 237, 242, 243, 245, 250, 915, 920, 928, 931, 934,
                936, 937, 8364,
            ],
        }
    }

    pub fn exists_in_array(&self, code: u16, array: Vec<u16>) -> bool {
        for e in array {
            if code == e {
                return true;
            }
        }
        false
    }

    pub fn validate_character(self, character: char) -> bool {
        let char_code = character as u16;
        self.exists_in_array(char_code, self.gsm_char_codes.clone())
    }

    pub fn validate_character_with_shift_table(self, character: char) -> bool {
        let char_code = character as u16;
        // concat GSM_TR_charCodes, GSM_ES_charCodes, GSM_PT_charCodes
        let mut shift_table = self.gsm_tr_char_codes.clone();
        shift_table.append(&mut self.gsm_es_char_codes.clone());
        shift_table.append(&mut self.gsm_pt_char_codes.clone());
        self.exists_in_array(char_code, shift_table)
    }

    pub fn validate_message_in_char_codes_list(&self,message: String,char_codes: Vec<u16>) -> bool {
        for c in message.chars() {
            if !self.exists_in_array(c as u16, char_codes.clone()) {
                return false;
            }
        }
        return true;
    }

    pub fn validate_message(self, message: String) -> bool {
        self.validate_message_in_char_codes_list(message, self.gsm_char_codes.clone())
    }

    pub fn validate_message_with_shift_table(self, message: String) -> bool {
        // var charCodes = [GSM_charCodes, GSM_TR_charCodes, GSM_ES_charCodes, GSM_PT_charCodes];
        let mut char_codes = self.gsm_char_codes.clone();
        char_codes.append(&mut self.gsm_tr_char_codes.clone());
        char_codes.append(&mut self.gsm_es_char_codes.clone());
        char_codes.append(&mut self.gsm_pt_char_codes.clone());
        for c in message.chars() {
            // validate_message_in_char_codes_list
            if self.validate_message_in_char_codes_list(c.to_string(), char_codes.clone()) {
                return true;
            }
        }
        return false;
    }

    pub fn validate_extended_character(self, character: char) -> bool {
        let char_code = character as u16;
        self.exists_in_array(char_code, self.gsme_char_codes.clone())
    }
    // validateExtendedCharacterWithShiftTable
    pub fn validate_extended_character_with_shift_table(self, character: char) -> bool {
        let char_code = character as u16;
        // var charCodes = GSMe_charCodes.concat(GSMe_TR_charCodes, GSMe_ES_charCodes, GSMe_PT_charCodes);
        let mut char_codes = self.gsme_char_codes.clone();
        char_codes.append(&mut self.gsme_tr_char_codes.clone());
        char_codes.append(&mut self.gsme_es_char_codes.clone());
        char_codes.append(&mut self.gsme_pt_char_codes.clone());
        self.exists_in_array(char_code, char_codes)
    }
}

impl Clone for GsmValidator {
    fn clone(&self) -> Self {
        GsmValidator {
            gsm_char_codes: self.gsm_char_codes.clone(),
            gsm_tr_char_codes: self.gsm_tr_char_codes.clone(),
            gsm_es_char_codes: self.gsm_es_char_codes.clone(),
            gsm_pt_char_codes: self.gsm_pt_char_codes.clone(),
            gsme_char_codes: self.gsme_char_codes.clone(),
            gsme_tr_char_codes: self.gsme_tr_char_codes.clone(),
            gsme_es_char_codes: self.gsme_es_char_codes.clone(),
            gsme_pt_char_codes: self.gsme_pt_char_codes.clone(),
        }
    }
}

// test
#[cfg(test)]
mod tests {
    use super::*;

    // Validating a message of every valid GSM characters
    #[test]
    fn gsm_validator() {
        let gsm_validator = GsmValidator::new();
        let message = "@Δ\x200¡P¿p£_!1AQaq$Φ\"2BRbr¥Γ#3CScsèΛ¤4DTdtéΩ%5EUeuùΠ&6FVfvìΨ\'7GWgwòΣ(8HXhxÇΘ)9IYiy\nΞ*:JZjzØ+;KÄkäøÆ,<LÖlö\ræ-=MÑmñÅß.>NÜnüåÉ/?O§oà|^€{}[~]\\f";
        assert_eq!(gsm_validator.validate_message(message.to_string()), true);
    }

    // Validating a message of one GSM character
    #[test]
    fn gsm_validator_one_character() {
        let gsm_validator = GsmValidator::new();
        let message = "@Δ\x200¡P¿p£_!1AQaq$Φ\"2BRbr¥Γ#3CScsèΛ¤4DTdtéΩ%5EUeuùΠ&6FVfvìΨ\'7GWgwòΣ(8HXhxÇΘ)9IYiy\nΞ*:JZjzØ+;KÄkäøÆ,<LÖlö\ræ-=MÑmñÅß.>NÜnüåÉ/?O§oà|^€{}[~]\\f";
        for c in message.chars() {
            assert_eq!(gsm_validator.clone().validate_message(c.to_string()), true);
        }
    }

    ///Validating a message of one non-GSM characters
    #[test]
    fn none_gsm_validator() {
        let gsm_validator = GsmValidator::new();
        let message = '\u{1F433}';
        assert_eq!(gsm_validator.validate_message(message.to_string()), false);
    }

    // Validating a message of every valid GSM Turkish shift table characters
    #[test]
    fn gsm_validator_with_shift_table_turkish() {
        let gsm_validator = GsmValidator::new();
        let message = "@£$¥€éùıòÇ\nĞğ\rÅåΔ_ΦΓΛΩΠΨΣΘΞŞşßÉ\x20!\"#¤%&\'()*+,-./0123456789:;<=>?İABCDEFGHIJKLMNOPQRSTUVWXYZÄÖÑÜ§çabcdefghijklmnopqrstuvwxyzäöñüàf^{}[~]|";
        assert_eq!(gsm_validator.validate_message_with_shift_table(message.to_string()), true);
    }

    // Validating a message of every valid GSM Spanish shift table characters
    #[test]
    fn gsm_validator_with_shift_table_spanish() {
        let gsm_validator = GsmValidator::new();
        let message = "@£$¥èéùìòÇ\nØø\rÅåΔ_ΦΓΛΩΠΨΣΘΞÆæßÉ\x20!\"#¤%&\'()*+,-./0123456789:;<=>?¡ABCDEFGHIJKLMNOPQRSTUVWXYZÄÖÑÜ§¿abcdefghijklmnopqrstuvwxyzäöñüàçf^{}\\[~]|ÁÍÓÚá€íóú";
        assert_eq!(gsm_validator.validate_message_with_shift_table(message.to_string()), true);
    }

    // Validating a message of every valid GSM Portuguese shift table characters
    #[test]
    fn gsm_validator_with_shift_table_portuguese() {
        let gsm_validator = GsmValidator::new();
        let message = "@£$¥êéúíóç\nÔô\rÁáΔ_ªÇÀ∞^\\€Ó|ÂâÊÉ\x20!\"#º%&\'()*+,-./0123456789:;<=>?ÍABCDEFGHIJKLMNOPQRSTUVWXYZÃÕÚÜ§~abcdefghijklmnopqrstuvwxyzãõ`üàfΦΓ^ΩΠΨΣΘ{}\\[~]|";
        assert_eq!(gsm_validator.validate_message_with_shift_table(message.to_string()), true);
    }

    // Validating a message of mixed valid GSM shift tables
    #[test]
    fn gsm_validator_with_shift_table_mixed() {
        let gsm_validator = GsmValidator::new();
        let message = "∞Ø";
        assert_eq!(gsm_validator.validate_message_with_shift_table(message.to_string()), true);
    }

    // Validating all GSM characters
    #[test]
    fn gsm_validator_all_characters() {
        let gsm_validator = GsmValidator::new();
        let message = "@Δ\x200¡P¿p£_!1AQaq$Φ\"2BRbr¥Γ#3CScsèΛ¤4DTdtéΩ%5EUeuùΠ&6FVfvìΨ\'7GWgwòΣ(8HXhxÇΘ)9IYiy\nΞ*:JZjzØ+;KÄkäøÆ,<LÖlö\ræ-=MÑmñÅß.>NÜnüåÉ/?O§oà|^€{}[~]\\f";
        for c in message.chars() {
            assert_eq!(gsm_validator.clone().validate_character(c), true);
        }
    }

    // Validating all GSM turkish characters
    #[test]
    fn gsm_validator_all_characters_turkish() {
        let gsm_validator = GsmValidator::new();
        let message = "@£$¥€éùıòÇ\nĞğ\rÅåΔ_ΦΓΛΩΠΨΣΘΞŞşßÉ\x20!\"#¤%&\'()*+,-./0123456789:;<=>?İABCDEFGHIJKLMNOPQRSTUVWXYZÄÖÑÜ§çabcdefghijklmnopqrstuvwxyzäöñüàf^{}[~]|";
        for c in message.chars() {
            assert_eq!(gsm_validator.clone().validate_character_with_shift_table(c), true);
        }
    }

    // Validating all GSM portuguese characters
    #[test]
    fn gsm_validator_all_characters_portuguese() {
        let gsm_validator = GsmValidator::new();
        let message = "@£$¥êéúíóç\nÔô\rÁáΔ_ªÇÀ∞^\\€Ó|ÂâÊÉ\x20!\"#º%&\'()*+,-./0123456789:;<=>?ÍABCDEFGHIJKLMNOPQRSTUVWXYZÃÕÚÜ§~abcdefghijklmnopqrstuvwxyzãõ`üàfΦΓ^ΩΠΨΣΘ{}\\[~]|";
        for c in message.chars() {
            assert_eq!(gsm_validator.clone().validate_character_with_shift_table(c), true);
        }
    }

    // Validating a non-GSM character
    #[test]
    fn gsm_validator_non_gsm_character() {
        let gsm_validator = GsmValidator::new();
        let message = '\u{1F433}';
        assert_eq!(gsm_validator.validate_character(message), false);
    }

}
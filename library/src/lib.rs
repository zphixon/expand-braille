pub fn paint(s: &str, replace: char) -> String {
    let mut string = String::new();
    for line in s.lines() {
        let mut line1 = String::new();
        let mut line2 = String::new();
        let mut line3 = String::new();
        let mut line4 = String::new();

        line.chars().for_each(|c| {
            let result = BrailleGlyph::try_from(c)
                .unwrap_or(BrailleGlyph::default())
                .expand(replace);
            line1.push_str(&result[0]);
            line2.push_str(&result[1]);
            line3.push_str(&result[2]);
            line4.push_str(&result[3]);
        });

        string.push_str(&format!("{line1}\n{line2}\n{line3}\n{line4}\n"));
    }
    string
}

#[derive(Debug, Default, Clone, Copy)]
pub struct BrailleGlyph {
    pub dot1: bool,
    pub dot2: bool,
    pub dot3: bool,
    pub dot4: bool,
    pub dot5: bool,
    pub dot6: bool,
    pub dot7: bool,
    pub dot8: bool,
}

impl TryFrom<char> for BrailleGlyph {
    type Error = ();
    fn try_from(c: char) -> Result<BrailleGlyph, ()> {
        let codepoint: u32 = c.into();

        if !(0x2800 <= codepoint && codepoint <= 0x28ff) {
            return Err(());
        }

        let hex_dots = codepoint - 0x2800;
        // lol TODO
        Ok(format!("{hex_dots:08b}").bytes().rev().enumerate().fold(
            BrailleGlyph::default(),
            |glyph, (i, bit)| {
                if bit == b'1' {
                    glyph.with(i + 1)
                } else {
                    glyph
                }
            },
        ))
    }
}

impl Into<char> for BrailleGlyph {
    fn into(self) -> char {
        let mut codepoint: u32 = 0x2800;
        codepoint |= if self.dot1 { 0b00000001 } else { 0 };
        codepoint |= if self.dot2 { 0b00000010 } else { 0 };
        codepoint |= if self.dot3 { 0b00000100 } else { 0 };
        codepoint |= if self.dot4 { 0b00001000 } else { 0 };
        codepoint |= if self.dot5 { 0b00010000 } else { 0 };
        codepoint |= if self.dot6 { 0b00100000 } else { 0 };
        codepoint |= if self.dot7 { 0b01000000 } else { 0 };
        codepoint |= if self.dot8 { 0b10000000 } else { 0 };
        unsafe { std::char::from_u32_unchecked(codepoint) }
    }
}

impl BrailleGlyph {
    pub fn with(self, dot: usize) -> BrailleGlyph {
        match dot {
            1 => BrailleGlyph { dot1: true, ..self },
            2 => BrailleGlyph { dot2: true, ..self },
            3 => BrailleGlyph { dot3: true, ..self },
            4 => BrailleGlyph { dot4: true, ..self },
            5 => BrailleGlyph { dot5: true, ..self },
            6 => BrailleGlyph { dot6: true, ..self },
            7 => BrailleGlyph { dot7: true, ..self },
            8 => BrailleGlyph { dot8: true, ..self },
            _ => self,
        }
    }

    pub fn paint(&self, replace: char) -> String {
        self.expand(replace)
            .iter()
            .fold(String::new(), |acc, line| format!("{acc}\n{line}"))
    }

    pub fn expand(&self, c: char) -> Vec<String> {
        let mut lines = Vec::with_capacity(4);
        let ab = format!("{c}{c}");
        let a = format!("{c} ");
        let b = format!(" {c}");

        let mut line1 = String::new();
        line1.push_str(if self.dot1 && self.dot4 {
            &ab
        } else if self.dot1 {
            &a
        } else if self.dot4 {
            &b
        } else {
            "  "
        });
        lines.push(line1);

        let mut line2 = String::new();
        line2.push_str(if self.dot2 && self.dot5 {
            &ab
        } else if self.dot2 {
            &a
        } else if self.dot5 {
            &b
        } else {
            "  "
        });
        lines.push(line2);

        let mut line3 = String::new();
        line3.push_str(if self.dot3 && self.dot6 {
            &ab
        } else if self.dot3 {
            &a
        } else if self.dot6 {
            &b
        } else {
            "  "
        });
        lines.push(line3);

        let mut line4 = String::new();
        line4.push_str(if self.dot7 && self.dot8 {
            &ab
        } else if self.dot7 {
            &a
        } else if self.dot8 {
            &b
        } else {
            "  "
        });
        lines.push(line4);

        lines
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test() {
        assert_eq!('⠀', BrailleGlyph::try_from('⠀').unwrap().into());
        assert_eq!('⠁', BrailleGlyph::try_from('⠁').unwrap().into());
        assert_eq!('⠂', BrailleGlyph::try_from('⠂').unwrap().into());
        assert_eq!('⠃', BrailleGlyph::try_from('⠃').unwrap().into());
        assert_eq!('⠄', BrailleGlyph::try_from('⠄').unwrap().into());
        assert_eq!('⠅', BrailleGlyph::try_from('⠅').unwrap().into());
        assert_eq!('⠆', BrailleGlyph::try_from('⠆').unwrap().into());
        assert_eq!('⠇', BrailleGlyph::try_from('⠇').unwrap().into());
        assert_eq!('⠈', BrailleGlyph::try_from('⠈').unwrap().into());
        assert_eq!('⠉', BrailleGlyph::try_from('⠉').unwrap().into());
        assert_eq!('⠊', BrailleGlyph::try_from('⠊').unwrap().into());
        assert_eq!('⠋', BrailleGlyph::try_from('⠋').unwrap().into());
        assert_eq!('⠌', BrailleGlyph::try_from('⠌').unwrap().into());
        assert_eq!('⠍', BrailleGlyph::try_from('⠍').unwrap().into());
        assert_eq!('⠎', BrailleGlyph::try_from('⠎').unwrap().into());
        assert_eq!('⠏', BrailleGlyph::try_from('⠏').unwrap().into());
        assert_eq!('⠐', BrailleGlyph::try_from('⠐').unwrap().into());
        assert_eq!('⠑', BrailleGlyph::try_from('⠑').unwrap().into());
        assert_eq!('⠒', BrailleGlyph::try_from('⠒').unwrap().into());
        assert_eq!('⠓', BrailleGlyph::try_from('⠓').unwrap().into());
        assert_eq!('⠔', BrailleGlyph::try_from('⠔').unwrap().into());
        assert_eq!('⠕', BrailleGlyph::try_from('⠕').unwrap().into());
        assert_eq!('⠖', BrailleGlyph::try_from('⠖').unwrap().into());
        assert_eq!('⠗', BrailleGlyph::try_from('⠗').unwrap().into());
        assert_eq!('⠘', BrailleGlyph::try_from('⠘').unwrap().into());
        assert_eq!('⠙', BrailleGlyph::try_from('⠙').unwrap().into());
        assert_eq!('⠚', BrailleGlyph::try_from('⠚').unwrap().into());
        assert_eq!('⠛', BrailleGlyph::try_from('⠛').unwrap().into());
        assert_eq!('⠜', BrailleGlyph::try_from('⠜').unwrap().into());
        assert_eq!('⠝', BrailleGlyph::try_from('⠝').unwrap().into());
        assert_eq!('⠞', BrailleGlyph::try_from('⠞').unwrap().into());
        assert_eq!('⠟', BrailleGlyph::try_from('⠟').unwrap().into());
        assert_eq!('⠠', BrailleGlyph::try_from('⠠').unwrap().into());
        assert_eq!('⠡', BrailleGlyph::try_from('⠡').unwrap().into());
        assert_eq!('⠢', BrailleGlyph::try_from('⠢').unwrap().into());
        assert_eq!('⠣', BrailleGlyph::try_from('⠣').unwrap().into());
        assert_eq!('⠤', BrailleGlyph::try_from('⠤').unwrap().into());
        assert_eq!('⠥', BrailleGlyph::try_from('⠥').unwrap().into());
        assert_eq!('⠦', BrailleGlyph::try_from('⠦').unwrap().into());
        assert_eq!('⠧', BrailleGlyph::try_from('⠧').unwrap().into());
        assert_eq!('⠨', BrailleGlyph::try_from('⠨').unwrap().into());
        assert_eq!('⠩', BrailleGlyph::try_from('⠩').unwrap().into());
        assert_eq!('⠪', BrailleGlyph::try_from('⠪').unwrap().into());
        assert_eq!('⠫', BrailleGlyph::try_from('⠫').unwrap().into());
        assert_eq!('⠬', BrailleGlyph::try_from('⠬').unwrap().into());
        assert_eq!('⠭', BrailleGlyph::try_from('⠭').unwrap().into());
        assert_eq!('⠮', BrailleGlyph::try_from('⠮').unwrap().into());
        assert_eq!('⠯', BrailleGlyph::try_from('⠯').unwrap().into());
        assert_eq!('⠰', BrailleGlyph::try_from('⠰').unwrap().into());
        assert_eq!('⠱', BrailleGlyph::try_from('⠱').unwrap().into());
        assert_eq!('⠲', BrailleGlyph::try_from('⠲').unwrap().into());
        assert_eq!('⠳', BrailleGlyph::try_from('⠳').unwrap().into());
        assert_eq!('⠴', BrailleGlyph::try_from('⠴').unwrap().into());
        assert_eq!('⠵', BrailleGlyph::try_from('⠵').unwrap().into());
        assert_eq!('⠶', BrailleGlyph::try_from('⠶').unwrap().into());
        assert_eq!('⠷', BrailleGlyph::try_from('⠷').unwrap().into());
        assert_eq!('⠸', BrailleGlyph::try_from('⠸').unwrap().into());
        assert_eq!('⠹', BrailleGlyph::try_from('⠹').unwrap().into());
        assert_eq!('⠺', BrailleGlyph::try_from('⠺').unwrap().into());
        assert_eq!('⠻', BrailleGlyph::try_from('⠻').unwrap().into());
        assert_eq!('⠼', BrailleGlyph::try_from('⠼').unwrap().into());
        assert_eq!('⠽', BrailleGlyph::try_from('⠽').unwrap().into());
        assert_eq!('⠾', BrailleGlyph::try_from('⠾').unwrap().into());
        assert_eq!('⠿', BrailleGlyph::try_from('⠿').unwrap().into());
        assert_eq!('⡀', BrailleGlyph::try_from('⡀').unwrap().into());
        assert_eq!('⡁', BrailleGlyph::try_from('⡁').unwrap().into());
        assert_eq!('⡂', BrailleGlyph::try_from('⡂').unwrap().into());
        assert_eq!('⡃', BrailleGlyph::try_from('⡃').unwrap().into());
        assert_eq!('⡄', BrailleGlyph::try_from('⡄').unwrap().into());
        assert_eq!('⡅', BrailleGlyph::try_from('⡅').unwrap().into());
        assert_eq!('⡆', BrailleGlyph::try_from('⡆').unwrap().into());
        assert_eq!('⡇', BrailleGlyph::try_from('⡇').unwrap().into());
        assert_eq!('⡈', BrailleGlyph::try_from('⡈').unwrap().into());
        assert_eq!('⡉', BrailleGlyph::try_from('⡉').unwrap().into());
        assert_eq!('⡊', BrailleGlyph::try_from('⡊').unwrap().into());
        assert_eq!('⡋', BrailleGlyph::try_from('⡋').unwrap().into());
        assert_eq!('⡌', BrailleGlyph::try_from('⡌').unwrap().into());
        assert_eq!('⡍', BrailleGlyph::try_from('⡍').unwrap().into());
        assert_eq!('⡎', BrailleGlyph::try_from('⡎').unwrap().into());
        assert_eq!('⡏', BrailleGlyph::try_from('⡏').unwrap().into());
        assert_eq!('⡐', BrailleGlyph::try_from('⡐').unwrap().into());
        assert_eq!('⡑', BrailleGlyph::try_from('⡑').unwrap().into());
        assert_eq!('⡒', BrailleGlyph::try_from('⡒').unwrap().into());
        assert_eq!('⡓', BrailleGlyph::try_from('⡓').unwrap().into());
        assert_eq!('⡔', BrailleGlyph::try_from('⡔').unwrap().into());
        assert_eq!('⡕', BrailleGlyph::try_from('⡕').unwrap().into());
        assert_eq!('⡖', BrailleGlyph::try_from('⡖').unwrap().into());
        assert_eq!('⡗', BrailleGlyph::try_from('⡗').unwrap().into());
        assert_eq!('⡘', BrailleGlyph::try_from('⡘').unwrap().into());
        assert_eq!('⡙', BrailleGlyph::try_from('⡙').unwrap().into());
        assert_eq!('⡚', BrailleGlyph::try_from('⡚').unwrap().into());
        assert_eq!('⡛', BrailleGlyph::try_from('⡛').unwrap().into());
        assert_eq!('⡜', BrailleGlyph::try_from('⡜').unwrap().into());
        assert_eq!('⡝', BrailleGlyph::try_from('⡝').unwrap().into());
        assert_eq!('⡞', BrailleGlyph::try_from('⡞').unwrap().into());
        assert_eq!('⡟', BrailleGlyph::try_from('⡟').unwrap().into());
        assert_eq!('⡠', BrailleGlyph::try_from('⡠').unwrap().into());
        assert_eq!('⡡', BrailleGlyph::try_from('⡡').unwrap().into());
        assert_eq!('⡢', BrailleGlyph::try_from('⡢').unwrap().into());
        assert_eq!('⡣', BrailleGlyph::try_from('⡣').unwrap().into());
        assert_eq!('⡤', BrailleGlyph::try_from('⡤').unwrap().into());
        assert_eq!('⡥', BrailleGlyph::try_from('⡥').unwrap().into());
        assert_eq!('⡦', BrailleGlyph::try_from('⡦').unwrap().into());
        assert_eq!('⡧', BrailleGlyph::try_from('⡧').unwrap().into());
        assert_eq!('⡨', BrailleGlyph::try_from('⡨').unwrap().into());
        assert_eq!('⡩', BrailleGlyph::try_from('⡩').unwrap().into());
        assert_eq!('⡪', BrailleGlyph::try_from('⡪').unwrap().into());
        assert_eq!('⡫', BrailleGlyph::try_from('⡫').unwrap().into());
        assert_eq!('⡬', BrailleGlyph::try_from('⡬').unwrap().into());
        assert_eq!('⡭', BrailleGlyph::try_from('⡭').unwrap().into());
        assert_eq!('⡮', BrailleGlyph::try_from('⡮').unwrap().into());
        assert_eq!('⡯', BrailleGlyph::try_from('⡯').unwrap().into());
        assert_eq!('⡰', BrailleGlyph::try_from('⡰').unwrap().into());
        assert_eq!('⡱', BrailleGlyph::try_from('⡱').unwrap().into());
        assert_eq!('⡲', BrailleGlyph::try_from('⡲').unwrap().into());
        assert_eq!('⡳', BrailleGlyph::try_from('⡳').unwrap().into());
        assert_eq!('⡴', BrailleGlyph::try_from('⡴').unwrap().into());
        assert_eq!('⡵', BrailleGlyph::try_from('⡵').unwrap().into());
        assert_eq!('⡶', BrailleGlyph::try_from('⡶').unwrap().into());
        assert_eq!('⡷', BrailleGlyph::try_from('⡷').unwrap().into());
        assert_eq!('⡸', BrailleGlyph::try_from('⡸').unwrap().into());
        assert_eq!('⡹', BrailleGlyph::try_from('⡹').unwrap().into());
        assert_eq!('⡺', BrailleGlyph::try_from('⡺').unwrap().into());
        assert_eq!('⡻', BrailleGlyph::try_from('⡻').unwrap().into());
        assert_eq!('⡼', BrailleGlyph::try_from('⡼').unwrap().into());
        assert_eq!('⡽', BrailleGlyph::try_from('⡽').unwrap().into());
        assert_eq!('⡾', BrailleGlyph::try_from('⡾').unwrap().into());
        assert_eq!('⡿', BrailleGlyph::try_from('⡿').unwrap().into());
        assert_eq!('⢀', BrailleGlyph::try_from('⢀').unwrap().into());
        assert_eq!('⢁', BrailleGlyph::try_from('⢁').unwrap().into());
        assert_eq!('⢂', BrailleGlyph::try_from('⢂').unwrap().into());
        assert_eq!('⢃', BrailleGlyph::try_from('⢃').unwrap().into());
        assert_eq!('⢄', BrailleGlyph::try_from('⢄').unwrap().into());
        assert_eq!('⢅', BrailleGlyph::try_from('⢅').unwrap().into());
        assert_eq!('⢆', BrailleGlyph::try_from('⢆').unwrap().into());
        assert_eq!('⢇', BrailleGlyph::try_from('⢇').unwrap().into());
        assert_eq!('⢈', BrailleGlyph::try_from('⢈').unwrap().into());
        assert_eq!('⢉', BrailleGlyph::try_from('⢉').unwrap().into());
        assert_eq!('⢊', BrailleGlyph::try_from('⢊').unwrap().into());
        assert_eq!('⢋', BrailleGlyph::try_from('⢋').unwrap().into());
        assert_eq!('⢌', BrailleGlyph::try_from('⢌').unwrap().into());
        assert_eq!('⢍', BrailleGlyph::try_from('⢍').unwrap().into());
        assert_eq!('⢎', BrailleGlyph::try_from('⢎').unwrap().into());
        assert_eq!('⢏', BrailleGlyph::try_from('⢏').unwrap().into());
        assert_eq!('⢐', BrailleGlyph::try_from('⢐').unwrap().into());
        assert_eq!('⢑', BrailleGlyph::try_from('⢑').unwrap().into());
        assert_eq!('⢒', BrailleGlyph::try_from('⢒').unwrap().into());
        assert_eq!('⢓', BrailleGlyph::try_from('⢓').unwrap().into());
        assert_eq!('⢔', BrailleGlyph::try_from('⢔').unwrap().into());
        assert_eq!('⢕', BrailleGlyph::try_from('⢕').unwrap().into());
        assert_eq!('⢖', BrailleGlyph::try_from('⢖').unwrap().into());
        assert_eq!('⢗', BrailleGlyph::try_from('⢗').unwrap().into());
        assert_eq!('⢘', BrailleGlyph::try_from('⢘').unwrap().into());
        assert_eq!('⢙', BrailleGlyph::try_from('⢙').unwrap().into());
        assert_eq!('⢚', BrailleGlyph::try_from('⢚').unwrap().into());
        assert_eq!('⢛', BrailleGlyph::try_from('⢛').unwrap().into());
        assert_eq!('⢜', BrailleGlyph::try_from('⢜').unwrap().into());
        assert_eq!('⢝', BrailleGlyph::try_from('⢝').unwrap().into());
        assert_eq!('⢞', BrailleGlyph::try_from('⢞').unwrap().into());
        assert_eq!('⢟', BrailleGlyph::try_from('⢟').unwrap().into());
        assert_eq!('⢠', BrailleGlyph::try_from('⢠').unwrap().into());
        assert_eq!('⢡', BrailleGlyph::try_from('⢡').unwrap().into());
        assert_eq!('⢢', BrailleGlyph::try_from('⢢').unwrap().into());
        assert_eq!('⢣', BrailleGlyph::try_from('⢣').unwrap().into());
        assert_eq!('⢤', BrailleGlyph::try_from('⢤').unwrap().into());
        assert_eq!('⢥', BrailleGlyph::try_from('⢥').unwrap().into());
        assert_eq!('⢦', BrailleGlyph::try_from('⢦').unwrap().into());
        assert_eq!('⢧', BrailleGlyph::try_from('⢧').unwrap().into());
        assert_eq!('⢨', BrailleGlyph::try_from('⢨').unwrap().into());
        assert_eq!('⢩', BrailleGlyph::try_from('⢩').unwrap().into());
        assert_eq!('⢪', BrailleGlyph::try_from('⢪').unwrap().into());
        assert_eq!('⢫', BrailleGlyph::try_from('⢫').unwrap().into());
        assert_eq!('⢬', BrailleGlyph::try_from('⢬').unwrap().into());
        assert_eq!('⢭', BrailleGlyph::try_from('⢭').unwrap().into());
        assert_eq!('⢮', BrailleGlyph::try_from('⢮').unwrap().into());
        assert_eq!('⢯', BrailleGlyph::try_from('⢯').unwrap().into());
        assert_eq!('⢰', BrailleGlyph::try_from('⢰').unwrap().into());
        assert_eq!('⢱', BrailleGlyph::try_from('⢱').unwrap().into());
        assert_eq!('⢲', BrailleGlyph::try_from('⢲').unwrap().into());
        assert_eq!('⢳', BrailleGlyph::try_from('⢳').unwrap().into());
        assert_eq!('⢴', BrailleGlyph::try_from('⢴').unwrap().into());
        assert_eq!('⢵', BrailleGlyph::try_from('⢵').unwrap().into());
        assert_eq!('⢶', BrailleGlyph::try_from('⢶').unwrap().into());
        assert_eq!('⢷', BrailleGlyph::try_from('⢷').unwrap().into());
        assert_eq!('⢸', BrailleGlyph::try_from('⢸').unwrap().into());
        assert_eq!('⢹', BrailleGlyph::try_from('⢹').unwrap().into());
        assert_eq!('⢺', BrailleGlyph::try_from('⢺').unwrap().into());
        assert_eq!('⢻', BrailleGlyph::try_from('⢻').unwrap().into());
        assert_eq!('⢼', BrailleGlyph::try_from('⢼').unwrap().into());
        assert_eq!('⢽', BrailleGlyph::try_from('⢽').unwrap().into());
        assert_eq!('⢾', BrailleGlyph::try_from('⢾').unwrap().into());
        assert_eq!('⢿', BrailleGlyph::try_from('⢿').unwrap().into());
        assert_eq!('⣀', BrailleGlyph::try_from('⣀').unwrap().into());
        assert_eq!('⣁', BrailleGlyph::try_from('⣁').unwrap().into());
        assert_eq!('⣂', BrailleGlyph::try_from('⣂').unwrap().into());
        assert_eq!('⣃', BrailleGlyph::try_from('⣃').unwrap().into());
        assert_eq!('⣄', BrailleGlyph::try_from('⣄').unwrap().into());
        assert_eq!('⣅', BrailleGlyph::try_from('⣅').unwrap().into());
        assert_eq!('⣆', BrailleGlyph::try_from('⣆').unwrap().into());
        assert_eq!('⣇', BrailleGlyph::try_from('⣇').unwrap().into());
        assert_eq!('⣈', BrailleGlyph::try_from('⣈').unwrap().into());
        assert_eq!('⣉', BrailleGlyph::try_from('⣉').unwrap().into());
        assert_eq!('⣊', BrailleGlyph::try_from('⣊').unwrap().into());
        assert_eq!('⣋', BrailleGlyph::try_from('⣋').unwrap().into());
        assert_eq!('⣌', BrailleGlyph::try_from('⣌').unwrap().into());
        assert_eq!('⣍', BrailleGlyph::try_from('⣍').unwrap().into());
        assert_eq!('⣎', BrailleGlyph::try_from('⣎').unwrap().into());
        assert_eq!('⣏', BrailleGlyph::try_from('⣏').unwrap().into());
        assert_eq!('⣐', BrailleGlyph::try_from('⣐').unwrap().into());
        assert_eq!('⣑', BrailleGlyph::try_from('⣑').unwrap().into());
        assert_eq!('⣒', BrailleGlyph::try_from('⣒').unwrap().into());
        assert_eq!('⣓', BrailleGlyph::try_from('⣓').unwrap().into());
        assert_eq!('⣔', BrailleGlyph::try_from('⣔').unwrap().into());
        assert_eq!('⣕', BrailleGlyph::try_from('⣕').unwrap().into());
        assert_eq!('⣖', BrailleGlyph::try_from('⣖').unwrap().into());
        assert_eq!('⣗', BrailleGlyph::try_from('⣗').unwrap().into());
        assert_eq!('⣘', BrailleGlyph::try_from('⣘').unwrap().into());
        assert_eq!('⣙', BrailleGlyph::try_from('⣙').unwrap().into());
        assert_eq!('⣚', BrailleGlyph::try_from('⣚').unwrap().into());
        assert_eq!('⣛', BrailleGlyph::try_from('⣛').unwrap().into());
        assert_eq!('⣜', BrailleGlyph::try_from('⣜').unwrap().into());
        assert_eq!('⣝', BrailleGlyph::try_from('⣝').unwrap().into());
        assert_eq!('⣞', BrailleGlyph::try_from('⣞').unwrap().into());
        assert_eq!('⣟', BrailleGlyph::try_from('⣟').unwrap().into());
        assert_eq!('⣠', BrailleGlyph::try_from('⣠').unwrap().into());
        assert_eq!('⣡', BrailleGlyph::try_from('⣡').unwrap().into());
        assert_eq!('⣢', BrailleGlyph::try_from('⣢').unwrap().into());
        assert_eq!('⣣', BrailleGlyph::try_from('⣣').unwrap().into());
        assert_eq!('⣤', BrailleGlyph::try_from('⣤').unwrap().into());
        assert_eq!('⣥', BrailleGlyph::try_from('⣥').unwrap().into());
        assert_eq!('⣦', BrailleGlyph::try_from('⣦').unwrap().into());
        assert_eq!('⣧', BrailleGlyph::try_from('⣧').unwrap().into());
        assert_eq!('⣨', BrailleGlyph::try_from('⣨').unwrap().into());
        assert_eq!('⣩', BrailleGlyph::try_from('⣩').unwrap().into());
        assert_eq!('⣪', BrailleGlyph::try_from('⣪').unwrap().into());
        assert_eq!('⣫', BrailleGlyph::try_from('⣫').unwrap().into());
        assert_eq!('⣬', BrailleGlyph::try_from('⣬').unwrap().into());
        assert_eq!('⣭', BrailleGlyph::try_from('⣭').unwrap().into());
        assert_eq!('⣮', BrailleGlyph::try_from('⣮').unwrap().into());
        assert_eq!('⣯', BrailleGlyph::try_from('⣯').unwrap().into());
        assert_eq!('⣰', BrailleGlyph::try_from('⣰').unwrap().into());
        assert_eq!('⣱', BrailleGlyph::try_from('⣱').unwrap().into());
        assert_eq!('⣲', BrailleGlyph::try_from('⣲').unwrap().into());
        assert_eq!('⣳', BrailleGlyph::try_from('⣳').unwrap().into());
        assert_eq!('⣴', BrailleGlyph::try_from('⣴').unwrap().into());
        assert_eq!('⣵', BrailleGlyph::try_from('⣵').unwrap().into());
        assert_eq!('⣶', BrailleGlyph::try_from('⣶').unwrap().into());
        assert_eq!('⣷', BrailleGlyph::try_from('⣷').unwrap().into());
        assert_eq!('⣸', BrailleGlyph::try_from('⣸').unwrap().into());
        assert_eq!('⣹', BrailleGlyph::try_from('⣹').unwrap().into());
        assert_eq!('⣺', BrailleGlyph::try_from('⣺').unwrap().into());
        assert_eq!('⣻', BrailleGlyph::try_from('⣻').unwrap().into());
        assert_eq!('⣼', BrailleGlyph::try_from('⣼').unwrap().into());
        assert_eq!('⣽', BrailleGlyph::try_from('⣽').unwrap().into());
        assert_eq!('⣾', BrailleGlyph::try_from('⣾').unwrap().into());
        assert_eq!('⣿', BrailleGlyph::try_from('⣿').unwrap().into());
    }
}

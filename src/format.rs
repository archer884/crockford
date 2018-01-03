use encoding;

#[derive(Copy, Clone, Debug)]
pub enum Case {
    Upper,
    Lower,
}

#[derive(Debug)]
/// An encoder with formatting options.
/// 
/// Values encoded using an instance of `Encoder` will be formatted with respect to the options
/// provided, e.g. capitalization, grouping of digits, and so forth.
pub struct Encoder { transform: fn(u8) -> u8 }

impl Encoder {
    pub fn new() -> Self {
        Self { transform: |u| u }
    }

    pub fn with_case(case: Case) -> Self {
        let transform = match case {
            Case::Upper => (|u| (u as char).to_ascii_uppercase() as u8) as fn(u8) -> u8,
            Case::Lower => (|u| (u as char).to_ascii_lowercase() as u8) as fn(u8) -> u8,
        };
        
        Self { transform }
    }

    pub fn encode(&self, n: u64) -> Formatter {
        let mut f = Formatter::new(self);
        encoding::encode_into(n, &mut f);
        f
    }
}

impl Default for Encoder {
    fn default() -> Self {
        Self::new()
    }
}

pub struct Formatter<'e> {
    encoder: &'e Encoder,
    len: usize,
    data: [u8; 13],
}

impl<'e> Formatter<'e> {
    pub fn new(encoder: &'e Encoder) -> Self {
        Formatter {
            encoder,
            len: 0,
            data: [0; 13],
        }
    }

    pub fn render(&self) -> String {
        let mut s = String::with_capacity(self.len);
        for idx in 0..self.len {
            s.push(self.data[idx] as char);
        }
        s
    }
}

impl<'e> encoding::Write for Formatter<'e> {
    fn write(&mut self, mut u: u8) {
        // FIXME: I believe this kind of transformation should be performed if and when the
        // formatter is realized rather than at write time. When we're writing, we should only
        // be writing.
        u = (self.encoder.transform)(u);

        // I'm not going to do an explicit bounds check here because #encode_into won't attempt to
        // write more than 13 bytes here. If you employ the #Write trait and then do the #left 
        // thing with it, that's your problem. Anyway, this isn't memory unsafe because indexed
        // access is implicitly checked, and you'll just get a panic if you try any dumbfuckery.
        self.data[self.len] = u;
        self.len += 1;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lowercase_encoder_works() {
        let encoder = Encoder::new();
        let result = encoder.encode(5111);

        assert_eq!("4zq", &*result.render());
    }

    #[test]
    fn uppercase_encoder_works() {
        let encoder = Encoder::with_case(Case::Upper);
        let result = encoder.encode(5111);

        assert_eq!("4ZQ", &*result.render());
    }
}

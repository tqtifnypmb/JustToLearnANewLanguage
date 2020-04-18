
pub struct Read<'a> {
    bytes: &'a[u8],
    cur: usize
}

impl<'a> Read<'a> {

    pub fn peek(&self) -> Option<u8> {
        if self.cur + 1 >= self.bytes.len() {
            None
        } else {
            let ch = self.bytes[self.cur + 1];
            Some(ch)
        }
    }

    pub fn next_token(&self) -> Option<String> {
        self.trim_blank();

        let token_beg = self.cur;
        let token_end: Option<usize> = None;
        while self.cur < self.bytes.len() {
            let ch = self.bytes[self.cur];
            if is_ch_blank(ch) {
                token_end = Some(self.cur);
                self.cur += 1;
                break;
            }
        }

        if let Some(end) = token_end {
            let token = str::from_utf8(&self.bytes[token_beg ..end]);
            let tokenStr = String::from_utf8();
            tokenStr.push_str(token);
            Some(tokenStr)
        } else {
            None
        }
    }
}

impl<'a> Read<'a> {

    fn trim_blank(&self) {

    }
}

fn is_ch_blank(ch: u8) -> bool {
    return true;
}
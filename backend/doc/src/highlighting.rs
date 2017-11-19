use syntect::dumps::from_binary;
use syntect::highlighting::ThemeSet;
use syntect::parsing::SyntaxSet;

thread_local!{
    pub static SYNTAX_SET: SyntaxSet = {
        let mut ss: SyntaxSet = from_binary(include_bytes!("../../../dumps/syntaxdump"));
        ss.link_syntaxes();
        ss
    };
}

lazy_static!{
    pub static ref THEME_SET: ThemeSet = from_binary(include_bytes!("../../../dumps/themedump"));
}

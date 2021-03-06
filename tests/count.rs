extern crate loc;

use loc::*;

// Because I can.  Seems bad though. Need a test runner with better output, or neeed to learn
macro_rules! test_count {
    ($path:expr, $exp:expr, $count:ident, $code:ident, $comment:ident, $blank:ident, $lines:ident) => (
        #[test]
        fn $count() {
            assert_eq!($exp, count($path));
        }

        #[test]
        fn $code() {
            assert_eq!($exp.code, count($path).code)
        }

        #[test]
        fn $comment() {
            assert_eq!($exp.comment, count($path).comment)
        }

        #[test]
        fn $blank() {
            assert_eq!($exp.blank, count($path).blank)
        }

        #[test]
        fn $lines() {
            assert_eq!($exp.lines, count($path).lines)
        }
    )
}

const PLASMA: &'static str = "tests/data/plasma.c";
const PLASMA_EXPECTED: Count = Count {
    code: 32032,
    blank: 8848,
    comment: 3792,
    lines: 44672,
};

test_count![PLASMA, PLASMA_EXPECTED, t_plasma_count, t_plasma_code, t_plasma_comment, t_plasma_blank, t_plasma_lines];

const FE: &'static str = "tests/data/fe25519.c";
const FE_EXPECTED: Count = Count {
    code: 278,
    blank: 51,
    comment: 8,
    lines: 278 + 51 + 8,
};

test_count![FE, FE_EXPECTED, test_fe_count, test_fe_code, test_fe_comment, test_fe_blank, test_fe_lines];

const EBC: &'static str = "tests/data/ebcdic.c";
const EBC_EXPECTED: Count = Count {
    code: 165,
    blank: 18,
    comment: 101,
    lines: 165 + 18 + 101,
};

test_count![EBC, EBC_EXPECTED, ebc_count, ebc_code, ebc_comment, evc_blank, ebc_lines];

const DUMB: &'static str = "tests/data/dumb.c";
const DUMB_EXPECTED: Count = Count {
    code: 2,
    blank: 0,
    comment: 3,
    lines: 5,
};
test_count![DUMB, DUMB_EXPECTED, dumb_count, dumb_code, dumb_comment, dumb_blank, dumb_lines];

const IPL: &'static str = "tests/data/ipl_funcs.c";
const IPL_EXPECTED: Count = Count {
    code: 25,
    blank: 6,
    comment: 43,
    lines: 25 + 6 + 43,
};
test_count![IPL, IPL_EXPECTED, ipl_count, ipl_code, ipl_comment, ipl_blank, ipl_lines];

const LUA: &'static str = "tests/data/lua.lua";
const LUA_EXPECTED: Count = Count {
    code: 7,
    blank: 1,
    comment: 8,
    lines: 7 + 8 + 1,
};
test_count![LUA, LUA_EXPECTED, lua_count, lua_code, lua_comment, lua_blank, lua_lines];

const RUBY: &'static str = "tests/data/test.rb";
const RUBY_EXPECTED: Count = Count {
    code: 2,
    blank: 0,
    comment: 2,
    lines: 2+2,
};
test_count![RUBY, RUBY_EXPECTED, ruby_count, ruby_code, ruby_comment, ruby_blank, ruby_lines];

const OCAML: &'static str = "tests/data/ocaml.ml";
const OCAML_EXPECTED: Count = Count {
    code: 3,
    blank: 4,
    comment: 6,
    lines: 3+4+6,
};
test_count![OCAML, OCAML_EXPECTED, ocaml_count, ocaml_code, ocaml_comment, ocaml_blank, ocaml_lines];

// single only
const ADA: &'static str = "tests/data/ada.ada";
const ADA_EXPECTED: Count = Count {
    code: 4,
    blank: 0,
    comment: 3,
    lines: 4+3,
};
test_count![ADA, ADA_EXPECTED, ada_count, ada_code, ada_comment, ada_blank, ada_lines];

const GHERKIN: &'static str = "tests/data/gherkin.feature";
const GHERKIN_EXPECTED: Count = Count {
    code: 8,
    blank: 2,
    comment: 2,
    lines: 8+2+2,
};
test_count![GHERKIN, GHERKIN_EXPECTED, gherkin_count, gherkin_code, gherkin_comment, gherkin_blank, gherkin_lines];
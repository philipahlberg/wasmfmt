use super::{Fmt, Formatter, Options};

use wast::{AssertExpression, Float32, Float64, Module, NanPattern, Wast, WastDirective, WastExecute, WastInvoke, parser::{parse, ParseBuffer}};

/// Format `.wast` source code.
pub fn fmt(source: &str, _options: Options) -> String {
    let buffer = ParseBuffer::new(source).expect("parse buffer");
    let wast = parse::<Wast>(&buffer).expect("parse");
    let mut formatter = Formatter::new();
    formatter.fmt(&wast);
    formatter.into()
}

impl<'src> Fmt for &Wast<'src> {
    fn fmt(&self, formatter: &mut Formatter) {
        for directive in &self.directives {
            formatter.start_line();
            formatter.fmt(directive);
            formatter.end_line();
        }
    }
}

impl<'src> Fmt for &WastDirective<'src> {
    fn fmt(&self, formatter: &mut Formatter) {
        match self {
            WastDirective::Module(module) => {
                formatter.fmt(module);
            },
            WastDirective::AssertReturn { exec, results, .. } => {
                let assert_return = AssertReturn {
                    exec,
                    results,
                };
                formatter.fmt(&assert_return);
            },
            WastDirective::AssertTrap { exec, message, .. } => {
                let assert_trap = AssertTrap {
                    exec,
                    message,
                };
                formatter.fmt(&assert_trap);
            },
            WastDirective::Invoke(invocation) => {
                formatter.fmt(invocation);
            },
            WastDirective::AssertExhaustion { call, message, .. } => {
                let assert_exhaustion = AssertExhaustion {
                    call,
                    message,
                };
                formatter.fmt(&assert_exhaustion);
            },
            WastDirective::AssertInvalid { module, message, .. } => {
                let assert_invalid = AssertInvalid {
                    module,
                    message,
                };
                formatter.fmt(&assert_invalid);
            },
            WastDirective::AssertMalformed { .. } => todo!(),
            WastDirective::AssertUnlinkable { .. } => todo!(),
            WastDirective::QuoteModule { .. } => todo!(),
            WastDirective::Register { .. } => todo!(),
        }
    }
}

struct AssertInvalid<'src> {
    module: &'src Module<'src>,
    message: &'src str,
}

impl<'src> Fmt for &AssertInvalid<'src> {
    fn fmt(&self, formatter: &mut Formatter) {
        formatter.write("(assert_invalid");
        formatter.end_line();
        formatter.indent();
        formatter.fmt(self.module);
        formatter.start_line();
        formatter.fmt(self.message);
        formatter.end_line();
        formatter.deindent();
        formatter.start_line();
        formatter.write(")");
    }
}

struct AssertExhaustion<'src> {
    call: &'src WastInvoke<'src>,
    message: &'src str,
}

impl<'src> Fmt for &AssertExhaustion<'src> {
    fn fmt(&self, formatter: &mut Formatter) {
        formatter.write("(assert_exhaustion ");
        formatter.fmt(self.call);
        formatter.write(" ");
        formatter.fmt(self.message);
        formatter.write(")");
    }
}

struct AssertTrap<'src> {
    exec: &'src WastExecute<'src>,
    message: &'src str,
}

impl<'src> Fmt for &AssertTrap<'src> {
    fn fmt(&self, formatter: &mut Formatter) {
        formatter.write("(assert_trap ");
        formatter.fmt(self.exec);
        formatter.write(" ");
        formatter.fmt(self.message);
        formatter.write(")");
    }
}

struct AssertReturn<'src> {
    exec: &'src WastExecute<'src>,
    results: &'src Vec<AssertExpression<'src>>,
}

impl<'src> Fmt for &AssertReturn<'src> {
    fn fmt(&self, formatter: &mut Formatter) {
        formatter.write("(assert_return ");
        formatter.fmt(self.exec);
        for assert_expression in self.results {
            formatter.write(" ");
            formatter.fmt(assert_expression);
        }
        formatter.write(")");
    }
}

impl<'src> Fmt for &WastExecute<'src> {
    fn fmt(&self, formatter: &mut Formatter) {
        match self {
            WastExecute::Invoke(invoke) => {
                formatter.fmt(invoke);
            },
            WastExecute::Module(_module) => {
                todo!()
            },
            WastExecute::Get { module: _, global: _ } => {
                todo!()
            }
        }
    }
}

impl<'src> Fmt for &WastInvoke<'src> {
    fn fmt(&self, formatter: &mut Formatter) {
        formatter.write("(invoke ");
        formatter.fmt(self.name);
        for expression in &self.args {
            formatter.write(" ");
            formatter.fmt(expression);
        }
        formatter.write(")");
    }
}

impl<'src> Fmt for &AssertExpression<'src> {
    fn fmt(&self, formatter: &mut Formatter) {
        match self {
            AssertExpression::I32(n) => {
                formatter.write("(i32.const ");
                formatter.fmt(*n);
                formatter.write(")");
            }
            AssertExpression::I64(n) => {
                formatter.write("(i64.const ");
                formatter.fmt(*n);
                formatter.write(")");
            }
            AssertExpression::F32(f) => {
                formatter.write("(f32.const ");
                formatter.fmt(f);
                formatter.write(")");
            }
            AssertExpression::F64(f) => {
                formatter.write("(f64.const ");
                formatter.fmt(f);
                formatter.write(")");
            }
            AssertExpression::V128(..) => unimplemented!(),
            AssertExpression::RefNull(..) => unimplemented!(),
            AssertExpression::RefExtern(..) => unimplemented!(),
            AssertExpression::RefFunc(..) => unimplemented!(),
            AssertExpression::LegacyArithmeticNaN => unimplemented!(),
            AssertExpression::LegacyCanonicalNaN => unimplemented!(),
        }
    }
}

impl<'src> Fmt for &NanPattern<Float32> {
    fn fmt(&self, formatter: &mut Formatter) {
        match self {
            NanPattern::Value(f) => {
                formatter.fmt(f);
            }
            NanPattern::ArithmeticNan => {
                formatter.write("nan:arithmetic");
            },
            NanPattern::CanonicalNan => {
                formatter.write("nan:canonical");
            },
        }
    }
}

impl<'src> Fmt for &NanPattern<Float64> {
    fn fmt(&self, formatter: &mut Formatter) {
        match self {
            NanPattern::Value(f) => {
                formatter.fmt(f);
            }
            NanPattern::ArithmeticNan => {
                formatter.write("nan:arithmetic");
            },
            NanPattern::CanonicalNan => {
                formatter.write("nan:canonical");
            },
        }
    }
}

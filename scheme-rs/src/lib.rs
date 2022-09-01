use error::LispResult;

use crate::{
    environment::{Env, Ports},
    lisp_val::LispVal,
    primitive_functions::primitive_functions,
};

pub mod environment;
pub mod error;
pub mod eval;
pub mod lisp_val;
pub mod numbers;
pub mod parser;
pub mod primitive_functions;

pub struct Thingus {
    env: Env,
    pub ports: Ports,
}

impl Thingus {
    pub fn new(signal: Box<dyn FnMut(&mut Vec<LispVal>)>) -> Self {
        let primitive_bindings = primitive_functions();
        let ports = Ports::new(signal);
        let env = Env::with_bindings(primitive_bindings, ports.clone());
        Thingus { env, ports }
    }
    pub fn eval(&self, input: &str) -> String {
        let parsed = parser::expression_list(input);
        match parsed {
            // TODO: Consume all input
            Ok((input, exprs)) => match eval::eval_expression_list(&self.env, exprs) {
                Ok(result) => result
                    .iter()
                    .filter(|&val| *val != LispVal::Void)
                    .map(|val| format!("{}", val))
                    .collect::<Vec<String>>()
                    .join("\n"),
                Err(err) => format!("{}", err),
            },
            // TODO
            Err(err) => format!("{}", err),
        }
    }
    pub fn eval_blah(&self, input: &str) -> LispResult<Vec<LispVal>> {
        // TODO: ??
        let (_, parsed) = parser::expression_list(input).unwrap();
        eval::eval_expression_list(&self.env, parsed)
    }
}

pub fn eval(input: &str, ports: Ports) -> String {
    let parsed = parser::expression_list(input);
    match parsed {
        // TODO: Consume all input
        Ok((input, exprs)) => {
            let primitive_bindings = primitive_functions();
            let env = Env::with_bindings(primitive_bindings, ports);

            match eval::eval_expression_list(&env, exprs) {
                Ok(result) => result
                    .iter()
                    .filter(|&val| *val != LispVal::Void)
                    .map(|val| format!("{}", val))
                    .collect::<Vec<String>>()
                    .join("\n"),
                Err(err) => format!("{}", err),
            }
        }
        // TODO
        Err(err) => format!("{}", err),
    }
}

#[cfg(test)]
mod tests {

    use std::{cell::RefCell, ops::Deref, rc::Rc};

    use super::*;

    fn noop(_port: &mut Vec<LispVal>) {}

    #[test]
    fn stuff_and_junk() {
        use super::*;
        let t = Thingus::new(Box::new(noop));
        assert_eq!(t.eval("\"foo\""), "\"foo\"");
        assert_eq!(t.eval("#\\a"), "#\\a");
        assert_eq!(t.eval("1729"), "1729");
        assert_eq!(t.eval("#(1 2 3)"), "#(1 2 3)");
    }

    #[test]
    fn blerg() {
        let t = Thingus::new(Box::new(noop));
        assert_eq!(t.eval("(string<? \"a\" \"b\" \"c\")"), "#t");
        assert_eq!(t.eval("(string<? \"a\" \"b\" \"c\" \"b\")"), "#f");
        assert_eq!(t.eval("(cdr '(1 2 3 4 5 6))"), "(2 3 4 5 6)");
        assert_eq!(t.eval("(cdr (cdr '(1 2 3 4 5 6)))"), "(3 4 5 6)");
        assert_eq!(t.eval("(cdr (cdr (cdr '(1 2 3 4 5 6))))"), "(4 5 6)");
        assert_eq!(t.eval("(cddr '(1 2 3 4 5 6))"), "(3 4 5 6)");
        assert_eq!(t.eval("(cdddr '(1 2 3 4 5 6))"), "(4 5 6)");

        assert_eq!(t.eval("(cdr '(1 2))"), "(2)");
        assert_eq!(t.eval("(car '(2))"), "2");
        assert_eq!(t.eval("(car (cdr '(1 2)))"), "2");
        assert_eq!(t.eval("(cadr '(1 2))"), "2");
    }

    #[test]
    fn test_little_schemer() {
        use std::env;
        use std::fs::File;
        use std::io::prelude::*;
        use std::path::Path;

        let t = Thingus::new(Box::new(noop));

        let schemer_tests = format!(
            "{}/tests/little_schemer.scm",
            env::var("CARGO_MANIFEST_DIR").unwrap()
        );

        let schemer_tests_path = Path::new(&schemer_tests);

        let mut file = String::new();
        File::open(&schemer_tests_path)
            .unwrap()
            .read_to_string(&mut file)
            .unwrap();

        for result in t
            .eval_blah(&file)
            .unwrap()
            .iter()
            .filter(|&val| *val != LispVal::Void)
        {
            assert_eq!(result, &LispVal::Bool(true));
        }
    }

    #[test]
    fn eval_lambda() {
        let t = Thingus::new(Box::new(noop));
        let input = concat!(
            "(define (foldl fn acc ls)",
            "    (if (null? ls)",
            "        acc",
            "        (foldl fn (fn acc (car ls)) (cdr ls))))",
            "(define (foldr fn acc ls)",
            "    (if (null? ls)",
            "        acc",
            "        (fn (car ls) (foldr fn acc (cdr ls)))))",
            "(define (map fn ls)",
            "    (foldr (lambda (x xs) (cons (fn x) xs)) '() ls))",
            "(define (filter fn ls)",
            "    (foldr (lambda (x xs) (if (fn x) (cons x xs) xs)) '() ls))",
            "(define (double n) (+ n n))",
            "(define (even?  n) (= 0 (modulo n 2)))",
            "(define (zero?  n) (= 0 n))",
            "(define (sub1   n) (- n 1))",
            "(define (not    b) (if b #f #t))",
            "(foldl  + 0    '(1 2 3 4 5))",
            "(map    double '(1 2 3 4 5))",
            "(filter even?  '(1 2 3 4 5 6 7 8 9 10))",
            "(let* ([a 5] [b (+ a 10)]) (+ b 20))",
            "(letrec",
            "    ([is-even? ",
            "        (lambda (n)",
            "            (if (zero? n) #t",
            "                (is-odd? (sub1 n))))]",
            "    [is-odd? ",
            "        (lambda (n)",
            "            (if (zero? n) #f",
            "                (is-even? (sub1 n))))])",
            "    (is-odd? 13))",
            "(letrec",
            "    ([is-even? ",
            "        (lambda (n)",
            "            (if (zero? n) #t",
            "                (is-odd? (sub1 n))))]",
            "    [is-odd? ",
            "        (lambda (n)",
            "            (if (zero? n) #f",
            "                (is-even? (sub1 n))))])",
            "    (is-odd? 12))",
        );

        assert_eq!(t.eval(input), "15\n(2 4 6 8 10)\n(2 4 6 8 10)\n35\n#t\n#f");
    }

    // #[test]
    fn stuff_vector_set() {
        let input = concat!(
            "(define temp (make-vector 5 'a))",
            "(vector-set! temp 3 9)",
            "temp",
        );
        let t = Thingus::new(Box::new(noop));
        assert_eq!(t.eval(input), "#(a a a 9 a)");

        let s = Rc::new(RefCell::new(vec![]));
        let cs = s.clone();
        let f = move |port: &mut Vec<LispVal>| {
            let mut s = cs.as_ref().borrow_mut();
            s.extend(port.drain(0..));
        };

        let input = "
            (define temp (make-vector 5 'a))
            (define (mut v i val) 
                (vector-set! v i val)
                (write temp)
                (write v)
                v)
            (define (foo a b) (write a) (write b))
            (foo '(1 2 3) 2)
            (define temp2 temp)
            (mut temp 4 6)
        ";
        let t = Thingus::new(Box::new(f));
        t.eval(input);
        let s = s.borrow();
        let s = s.deref();
        let result = s
            .iter()
            .map(|val| format!("{}", val))
            .collect::<Vec<String>>()
            .join("\n");
        assert_eq!(
            result,
            "(1 2 3)\n2\n#(a a a a 6)\n#(a a a a 6)\n#(a a a a 6)"
        );

        // (let ((vec (vector 0 '(2 2 2 2) "Anna")))
        //   (vector-set! vec 1 '("Sue" "Sue"))
        //   vec)
        //      =>  #(0 ("Sue" "Sue") "Anna")
    }
}

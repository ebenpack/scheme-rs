(define (test a b) (eq? a b))

(define (exp base power)
    (cond
        [(zero? power) 1]
        [(eq? power 1) base]
        [else (* base (exp base (- power 1)))]))

(define (numbered? aexp)
    (cond
        [(atom? aexp) (number? aexp)]
        [(or
            (eq? (cadr aexp) '+)
            (eq? (cadr aexp) 'x)
            (eq? (cadr aexp) '^))
            (and (numbered? (car aexp)) (numbered? (caddr aexp)))]
        [else #f]))

(test (numbered? '(3 + (4 x 5))) #t)
(test (numbered? '(3 + (4 ^ 5))) #t)
(test (numbered? '(2 x sausage)) #f)

(define (first-sub-exp aexp)
    (car aexp))

(define (second-sub-exp aexp)
    (caddr aexp))

(define (operator aexp)
    (cadr aexp))

(define (value nexp)
    (cond
        [(atom? nexp) nexp]
        [(eq? (operator nexp) '+)
            (+ (value (first-sub-exp nexp)) (value (second-sub-exp nexp)))]
        [(eq? (operator nexp) 'x)
            (* (value (first-sub-exp nexp)) (value (second-sub-exp nexp)))]
        [(eq? (operator nexp) '^)
            (exp (value (first-sub-exp nexp)) (value (second-sub-exp nexp)))]))

(test (value '(1 + (3 x 4))) 13)

;;
'OK
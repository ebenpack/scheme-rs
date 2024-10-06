(define (test a b) (equal? a b))

(define (rember-f test?)
    (lambda (a l)
        (cond
            [(null? l) '()]
            [(test? (car l) a)
                (cdr l)]
            [else
                (cons (car l) ((rember-f test?) a (cdr l)))])))

(test
    ((rember-f eq?) 'tuna '(shrimp salad and tuna salad))
    '(shrimp salad and salad))

(define (insert-g insert)
    (lambda (test?)
        (lambda (new old l)
            (cond
                [(null? l) '()]
                [(test? (car l) old)
                    (insert new old (cdr l))]
                [else
                    (cons (car l) ((insert-g test? insert) new old (cdr l)))]))))

(define insertL-f
    (insert-g (lambda (new old l) (cons new (cons old l)))))

(define insertR-f
    (insert-g (lambda (new old l) (cons old (cons new l)))))

;;
'OK
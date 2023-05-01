(define (test a b) (eq? a b))

(define (pick n lat)
    (cond
        [(= n 1)
            (car lat)]
        [else
            (pick (- n 1) (cdr lat))]))

(define (keep-looking a target lat)
    (cond
        [(number? target)
            (keep-looking a (pick target lat) lat)]
        [(atom? target)
            (eq? a target)]
        [else #f]))

(define (looking a lat)
    (keep-looking a (pick 1 lat) lat))

(test
    (looking 'caviar '(6 2 4 caviar 5 7 3))
    #t)

(test
    (looking 'caviar '(6 2 grits caviar 5 7 3))
    #f)

(define Y
    (lambda (le)
        ((lambda (f) (f f))
        (lambda (f)
            (le (lambda (x) ((f f) x)))))))

(define fact
    (lambda (recurs)
        (lambda (x)
            (cond
                [(= 0 x) 1]
                [else (* x (recurs (- x 1)))]))))

(test
    ((Y fact) 3)
    6)

;;
'OK
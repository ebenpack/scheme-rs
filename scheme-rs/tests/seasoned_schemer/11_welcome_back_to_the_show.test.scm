(define (test a b) (eq? a b))

(define (two-in-a-row lat)
    (cond
        [(null? lat) #f]
        [(null? (cdr lat)) #f]
        [else
            (or 
                (eq? (car lat) (cadr lat))
                (two-in-a-row (cdr lat)))]))

(test
    (two-in-a-row
        '(Italian sardines sardines spaghetti parsley))
    #t)

(test
    (two-in-a-row
        '(Italian sardines more sardines spaghetti parsley))
    #f)

;;
'OK
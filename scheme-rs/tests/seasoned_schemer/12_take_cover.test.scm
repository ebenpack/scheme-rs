(define (test a b) (eq? a b))

(define multirember
    (lambda (a lat)
        (letrec
            ((mr (lambda (lat)
                (cond
                    ((null? lat) (quote ()))
                    ((eq? a (car lat))
                        (mr (cdr lat)))
                    (else 
                        (cons (car lat)
                            (mr (cdr lat))))))))
        (mr lat))))

(test
    (multirember
        'pie '(apple custard pie linzer torte))
    '(apple custard linzer torte))

(define recmember?
    (lambda (a lat)
    (letrec ((yes? (lambda (l)
        (cond 
            ((null? l) #f)
            ((eq? (car l) a) #t)
            (else (yes? (cdr l)))))))
    (yes? lat))))

(test
    (recmember?
        'ice '(salad greens with pears brie cheese frozen yogurt))
    #f)

(test
    (recmember?
        'frozen '(salad greens with pears brie cheese frozen yogurt))
    #t)

(define union
    (lambda (set1 set2)
    (cond
        ((null? set1) set2)
        ((member? (car set1) (union (cdr set1) set2)))
        (else (cons (car set1) (union (cdr set1) set2))))))

;;
'OK
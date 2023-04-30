(define (test a b) (eq? a b))

(define (rember* a l)
    (cond
        [(null? l) '()]
        [(atom? (car l))
            (cond
                [(eq? (car l) a)
                    (rember* a (cdr l))]
                [else
                    (cons (car l) (rember* a (cdr l)))])]
        [else
            (cons (rember* a (car l)) (rember* a (cdr l)))]))

(test
    (rember* 'cup '((coffee) cup ((tea) cup) (and (hick)) cup))
    '((coffee) ((tea)) (and (hick))))

(define (insertR* new old l)
    (cond
        [(null? l) '()]
        [(atom? (car l))
            (cond
                [(eq? old (car l))
                    (cons old (cons new (insertR* new old (cdr l))))]
                [else
                    (cons (car l) (insertR* new old (cdr l)))])]
        [else
            (cons (insertR* new old (car l)) (insertR* new old (cdr l)))]))

(test
    (insertR* 'roast 'chuck
        '((how much (wood)) could ((a (wood) chuck)) (((chuck))) (if (a) ((wood chuck))) could chuck wood))
        '((how much (wood)) could ((a (wood) chuck roast)) (((chuck roast))) (if (a) ((wood chuck roast))) could chuck roast wood))

(define (occur* a l)
  (cond
    [(null? l) 0]
    [(atom? (car l))
      (cond
        [(eq? (car l) a)
            (+ 1 (occur* a (cdr l)))]
        [else
            (occur* a (cdr l))])]
    [else (+ (occur* a (car l)) (occur* a (cdr l)))]))

(test (occur* 'banana '((banana)
    (split ((((banana ice)))
    (cream (banana))
    sherbet))
    (banana)
    (bread)
    (banana brandy))) 5)

(define (subst* new old l)
  (cond
    [(null? l) '()]
    [(atom? (car l))
      (cond
        [(eq? (car l) old)
            (cons new (subst* new old (cdr l)))]
        [else
            (cons (car l) (subst* new old (cdr l)))])]
    [else
        (cons (subst* new old (car l)) (subst* new old (cdr l)))]))

(test
    (subst* 'orange 'banana
        '((banana)
            (split ((((banana ice)))
                (cream (banana))
                sherbet))
            (banana)
            (bread)
            (banana brandy)))
        '((orange)
            (split ((((orange ice)))
                (cream (orange))
                sherbet))
            (orange)
            (bread)
            (orange brandy)))

(define (member* a l)
    (cond
        [(null? l) #f]
        [(atom? (car l))
            (or (eq? (car l) a) (member* a (cdr l)))]
        [else
            (or (member* a (car l)) (member* a (cdr l)))]))

(test
    (member* 'chips '((potato) (chips ((with) fish) (chips))))
    #t)

(define (leftmost l)
    (cond
        [(atom? (car l))
            (car l)]
        [else
            (leftmost (car l))]))

(test
    (leftmost '(((hot) (tuna (and))) cheese))
    'hot)

(define (eqlist? l1 l2)
    (cond
        [(and (null? l1) (null? l2)) #t]
        [(or (null? l1) (null? l2)) #f]
        [(and
            (atom? (car l1))
            (atom? (car l2))
            (eq? (car l1) (car l2)))
                (eqlist? (cdr l1) (cdr l2))]
        [(and
            (pair? (car l1))
            (pair? (car l2))
            (eqlist? (car l1) (car l2)))
                (eqlist? (cdr l1) (cdr l2))]
        [else #f]))

(test
    (eqlist?
        '(strawberry ice cream)
        '(strawberry ice cream))
    #t)

(test
    (eqlist?
        '(strawberry ice cream)
        '(strawberry cream ice))
    #f)

(test
    (eqlist?
        '(banana ((split)))
        '((banana) (split)))
    #f)

(test
    (eqlist?
        '(beef ((sausage)) (and (soda)))
        '(beef ((salami)) (and (soda))))
    #f)

(test
    (eqlist?
        '(beef ((sausage)) (and (soda)))
        '(beef ((sausage)) (and (soda))))
    #t)

;;
'OK
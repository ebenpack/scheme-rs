(define (test a b) (equal? a b))

(define (set? lat)
    (cond
    [(null? lat) #t]
    [else
        (and
            (not (member? (car lat) (cdr lat)))
            (set? (cdr lat)))]))

(test (set? '()) #t)
(test (set? '(a b c)) #t)
(test (set? '(a b c d e a)) #f)

(define (makeset lat)
    (cond
        [(null? lat)
            '()]
        [(member? (car lat) (cdr lat))
            (makeset (cdr lat))]
        [else
            (cons (car lat) (makeset (cdr lat)))]))

(test
    (makeset '(apple peach pear peach plum apple lemon peach))
    '(pear plum apple lemon peach))

(define (subset? s1 s2)
    (cond
        [(null? s1)
            #t]
        [(member? (car s1) s2)
            (subset? (cdr s1) s2)]
        [else #f]))

(test
    (subset?
        '(5 chicken wings)
        '(5 hamburgers 2 pieces fried chicken and light duckling wings))
    #t)
(test
    (subset?
        '(4 pounds of horseradish)
        '(four pounds chicken and 5 ounces horseradish))
    #f)

(define (eqset? s1 s2)
    (and (subset? s1 s2) (subset? s2 s1)))

(test
    (eqset?
        '(6 large chickens with wings)
        '(6 chickens with large wings))
    #t)

(define (intersect? s1 s2)
    (cond
        [(null? s1) #f]
        [(member? (car s1) s2) #t]
        [else
            (intersect? (cdr s1) s2)]))

(test
    (intersect?
        '(stewed tomatoes and macaroni)
        '(macaroni and cheese))
    #t)

(define (intersect s1 s2)
    (cond
        [(null? s1) '()]
        [(member? (car s1) s2)
            (cons (car s1) (intersect (cdr s1) s2))]
        [else
            (intersect (cdr s1) s2)]))

(test
    (intersect
        '(stewed tomatoes and macaroni)
        '(macaroni and cheese))
    '(and macaroni))

(define (union s1 s2)
    (cond
        [(null? s1) s2]
        [(member? (car s1) s2)
            (union (cdr s1) s2)]
        [else
            (cons (car s1) (union (cdr s1) s2))]))

(test
    (union
        '(stewed tomatoes and macaroni casserole)
        '(macaroni and cheese))
    '(stewed tomatoes casserole macaroni and cheese))

(define (intersectall l-set)
    (cond
        [(null? l-set) '()]
        [(null? (cdr l-set)) (car l-set)]
        [else
            (intersect (car l-set) (intersectall (cdr l-set)))]))

(test
    (intersectall
        '((a b c) (c a de) (e f g h a b)))
        '(a))

(test
    (intersectall
    '((6 pears and)
        (3 peaches and 6 peppers)
        (8 pears and 6 plums)
        (and 6 prunes with some apples)))
    '(6 and))

;;
'OK
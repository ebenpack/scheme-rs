(define (test a b) (eq? a b))

(test (atom? 'atom) #t)
(test (atom? 'turkey) #t)
(test (atom? 1729) #t)
(test (atom? #\a) #t)
(test (atom? '(Harry had a heap of apples)) #f)
(test (atom? (car '(Harry had a heap of apples))) #t)
(test (atom? (cdr '(Harry had a heap of apples))) #f)
(test (atom? (car (cdr '(swing low sweet cherry oat)))) #t)
(test (atom? (car (cdr '(swing (low sweet) cherry oat)))) #f)
(test (atom? (cdr '(Harry))) #f)
(test (list? '(atom)) #t)
(test (list? '(atom turkey or)) #t)
(test (list? '((atom turkey) or)) #t)
(test (list? '()) #t)
(test (atom? '()) #f)
(test (list? '(() () () ())) #t)
(test (list? '(1 . 2)) #f)
(test (list? '(1 . (2))) #t)
(test (list? '(1 . ())) #t)

(test (car '(a b c)) 'a)
(test (car '((a b c) x y z)) '(a b c))
(test (car '(((hotdogs)) (and) (pickle) relish)) '((hotdogs)))
(test (car (car '(((hotdogs)) (and) (pickle) relish))) '(hotdogs))
(test (cdr '(a b c)) '(b c))
(test (cdr '((a b c) x y z)) '(x y z))
(test (cdr '(hamburger)) '())
(test (cdr '((x) t r)) '(t r))
(test (car (cdr '((b) (x y) ((e))))) '(x y))
(test (cdr (cdr '((b) (x y) ((e))))) '(((e))))

(test (cons 'peanut '(butter and jelly)) '(peanut butter and jelly))
(test (cons '(banana and) '(peanut butter and jelly)) '((banana and) peanut butter and jelly))
(test (cons '((help) this) '(is very ((hard) to learn))) '(((help) this) is very ((hard) to learn)))
(test (cons '(a b (c)) '()) '((a b (c))))
(test (cons 'a '()) '(a))
(test (cons 'a (car '((b) c d))) '(a b))
(test (cons 'a (cdr '((b) c d))) '(a c d))

(test (null? '()) #t)
(test (null? '(a b c)) #f)

(test (eq? 'Harry 'Harry) #t)
(test (eq? 'butter 'margarine) #f)
(test (eq? (car '(Mary had a little lamb chop)) 'Mary) #t)
(test (eq? (car '(beans beans we need jelly beans)) (car (cdr '(beans beans we need jelly beans)))) #t)

(define lat?
  (lambda (l)
    (cond
      ((null? l) #t)
      ((atom? (car l)) (lat? (cdr l)))
      (else #f))))

(test (lat? '(Jack Sprat could eat no chicken fat)) #t)
(test (lat? '((Jack) Sprat could eat no chicken fat)) #f)
(test (lat? '(Jack (Sprat could) eat no chicken fat)) #f)
(test (lat? '()) #t)

(test (or (null? '()) (atom? '(d e f g))) #t)
(test (or (null? '(a b c)) (null? '())) #t)
(test (or (null? '(a b c)) (null? '(atom))) #f)

(test (member? 'tea '(coffee tea or milk)) #t)
(test (member? 'poached '(fried eggs and scrambled eggs)) #f)

(define (occur* a l)
  (cond
    ((null? l) 0)
    ((atom? (car l))
      (cond
        ((eq? (car l) a) (+ 1 (occur* a (cdr l))))
        (else (occur* a (cdr l)))
        ))
    (else (+ (occur* a (car l)) (occur* a (cdr l))))))

(test (occur* 'banana '((banana)
    (split ((((banana ice)))
    (cream (banana))
    sherbet))
    (banana)
    (bread)
    (banana brandy))) 5)

(define (subst* new old l)
  (cond
    ((null? l) '())
    ((atom? (car l))
      (cond
        ((eq? (car l) old) (cons new (subst* new old (cdr l))))
        (else (cons (car l) (subst* new old (cdr l))))))
    (else (cons (subst* new old (car l)) (subst* new old (cdr l))))))

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

(define (calculate l)
  (cond 
    ((atom? l) l)
    ((and (atom? car l) (atom? (cdr l))) )
    ((list? (car l)) (calculate (car l)))
    ((eq? (length l) 1) (car l))
    ((eq? (car (cdr l)) '+) (+ (car l) (calculate (cddr l))))
    ((eq? (car (cdr l)) 'x) (* (car l) (calculate (cddr l))))
    ))


(test (calculate '(1 + (2 x 5))) 11)
(test (calculate '(1 + 2 x 5)) 11)

;;
'OK
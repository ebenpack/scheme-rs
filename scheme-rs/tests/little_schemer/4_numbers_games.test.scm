(define (test a b) (eq? a b))

(test (atom? 1729) #t)
(test (number? -3) #t)
;; (test (number? 3.14159) #t)
(test (+ 46 12) 58)

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
    ((eq? (car (cdr l)) 'x) (* (car l) (calculate (cddr l))))))


(test (calculate '(1 + (2 x 5))) 11)
(test (calculate '(1 + 2 x 5)) 11)

;;
'OK
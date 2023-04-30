(define (test a b) (eq? a b))

(define (add1 n) (+ n 1))
(define (sub1 n) (- n 1))

(define (add m n)
  (if (zero? m)
    n
    (add (sub1 m) (add1 n))))
(define (sub m n)
  (if (zero? n)
    m
    (sub (sub1 m) (sub1 n))))

(define (tup+ tup1 tup2)
  (cond
    ((and (null? tup1) (null? tup2)) '())
    (else (cons (+ (car tup1) (car tup2)) (tup+ (cdr tup1) (cdr tup2))))))

(test (atom? 1729) #t)
(test (number? -3) #t)
(test (number? 3.14159) #t)
(test (+ 46 12) 58)
(test (add1 67) 68)
(test (sub1 67) 66)
(test (zero? -0.0) #t)
(test (zero? 0) #t)
(test (zero? 1729) #f)
(test (- 14 3) 11)
(test (- 17 9) 8)
(test (* 5 3) 15)
(test (* 13 4) 52)
(test (tup+ '(2 3) '(4 6)) '(6 9))

;;
'OK
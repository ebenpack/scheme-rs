;; TODO - Import test function
(define (test a b) (equal? a b))

; list of atoms?
(define lat?
  (lambda (l)
    (cond
      [(null? l) #t]
      [(atom? (car l)) (lat? (cdr l))]
      [else #f])))

(test (lat? '(Jack Sprat could eat no chicken fat)) #t)
(test (lat? '((Jack) Sprat could eat no chicken fat)) #f)
(test (lat? '(Jack (Sprat could) eat no chicken fat)) #f)
(test (lat? '()) #t)

(test (or (null? '()) (atom? '(d e f g))) #t)
(test (or (null? '(a b c)) (null? '())) #t)
(test (or (null? '(a b c)) (null? '(atom))) #f)

(test (member? 'tea '(coffee tea or milk)) #t)
(test (member? 'poached '(fried eggs and scrambled eggs)) #f)

;;
'OK
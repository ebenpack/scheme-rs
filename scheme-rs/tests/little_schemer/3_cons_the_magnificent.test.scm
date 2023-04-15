;; TODO - Import test function
(define (test a b) (eq? a b))

(define (rember a l)
  (cond
    ((null? l) '())
    ((eq? a (car l)) (cdr l))
    (else (cons (car l) (rember a (cdr l))))))

(test
  (rember 'mint '(lamb chops and mint jelly))
  '(lamb chops and jelly))
(test
  (rember 'mint '(lamb chops and mint flavored mint jelly))
  '(lamb chops and flavored mint jelly))
(test
  (rember 'cup '(coffee cup tea cup and hick cup))
  '(coffee tea cup and hick cup))

(define (firsts l)
  (cond
    ((null? l) '())
    (else (cons (car (car l)) (firsts (cdr l))))))

(test 
  (firsts '((apple peach pumpkin) (plum pear cherry) (grape raisin pea) (bean carrot eggplant)))
  '(apple plum grape bean))
(test
  (firsts '((a b) (c d) (e f)))
  '(a c e))
(test (firsts '()) '())

(define (seconds l)
  (cond
    ((null? l) '())
    (else (cons (car (cdr (car l))) (seconds (cdr l))))))

(test
  (seconds '((a b) (c d) (e f)))
  '(b d f))

(define (insertR new old l)
  (cond
    ((null? l) '())
    ((eq? old (car l)) (cons old (cons new (cdr l))))
    (else (cons (car l) (insertR new old (cdr l))))))

(test
  (insertR 'topping 'fudge '(ice cream with fudge for dessert))
  '(ice cream with fudge topping for dessert))
;; Our scheme doesn't currently have support for diacritics like ñ ¯\_(ツ)_/¯
(test
  (insertR 'jalapeno 'and '(tacos tamales and salsa))
  '(tacos tamales and jalapeno salsa))

(define (insertL new old l)
  (cond
    ((null? l) '())
    ((eq? old (car l)) (cons new l))
    (else (cons (car l) (insertL new old (cdr l))))))

(test
  (insertL 'topping 'fudge '(ice cream with fudge for dessert))
  '(ice cream with topping fudge for dessert))
(test
  (insertL 'jalapeno 'and '(tacos tamales and salsa))
  '(tacos tamales jalapeno and salsa))

(define (subst new old l)
  (cond
    ((null? l) '())
    ((eq? old (car l)) (cons new (cdr l)))
    (else (cons (car l) (subst new old (cdr l))))))

(test
  (subst 'topping 'fudge '(ice cream with fudge for dessert))
  '(ice cream with topping for dessert))

(define (subst2 new o1 o2 l)
  (cond
    ((null? l) '())
    ((or (eq? o1 (car l)) (eq? o2 (car l))) (cons new (cdr l)))
    (else (cons (car l) (subst2 new o1 o2 (cdr l))))))

(test
  (subst2 'vanilla 'chocolate 'banana '(banana ice cream with chocolate topping))
  '(vanilla ice cream with chocolate topping))

(define (multirember a l)
  (cond
    ((null? l) '())
    ((eq? a (car l)) (multirember a (cdr l)))
    (else (cons (car l) (multirember a (cdr l))))))

(test
  (multirember 'cup '(coffee cup tea cup and hick cup))
  '(coffee tea and hick))

(define (multiinsertR new old l)
  (cond
    ((null? l) '())
    ((eq? old (car l)) (cons old (cons new (multiinsertR new old (cdr l)))))
    (else (cons (car l) (multiinsertR new old (cdr l))))))

(test
  (multiinsertR 'new 'old '(old one old two old three))
  '(old new one old new two old new three))

(define (multiinsertL new old l)
  (cond
    ((null? l) '())
    ((eq? old (car l)) (cons new (cons old (multiinsertL new old (cdr l)))))
    (else (cons (car l) (multiinsertL new old (cdr l))))))

(test
  (multiinsertL 'new 'old '(old one old two old three))
  '(new old one new old two new old three))

(define (multisubst new old l)
  (cond
    ((null? l) '())
    ((eq? old (car l)) (cons new (multisubst new old (cdr l))))
    (else (cons (car l) (multisubst new old (cdr l))))))
  
(test
  (multisubst 'new 'old '(old thing one old thing two old three))
  '(new thing one new thing two new three))

;;
'OK
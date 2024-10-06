(define (test a b) (equal? a b))

(define first
  (lambda (p)
    (car p)))

(define second
  (lambda (p)
    (car (cdr p))))

(define build
  (lambda (s1 s2)
    (cons s1 (cons s2 '()))))

(define (lookup-in-entry name entry entry-f)
    (lookup-in-entry-help name (first entry) (second entry) entry-f))

(define (lookup-in-entry-help name names values entry-f)
    (cond
        [(null? names) (entry-f name)]
        [(eq? name (car names))
            (car values)]
        [else
            (lookup-in-entry-help name (cdr names) (cdr values) entry-f)]))

(define (lookup-in-table name table table-f)
    (cond
        [(null? table) (table-f name)]
        [else
            (lookup-in-entry name (car table)
                (lambda (name) (lookup-in-table name (cdr table) table-f)))]))

;;
'OK
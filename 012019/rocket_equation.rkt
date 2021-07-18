#lang racket

(define (fuel l)
  (- (floor (/ l 3)) 2))

(define (whole-fuel l [acc 0])
  (define nl (fuel l))
  (cond
    [(>= 0 nl) acc] 
    [else 
      (whole-fuel nl (+ acc nl))]))

(define (main)
(with-input-from-file "input"
    (lambda ()
      (for/sum ([l (in-lines)])
        (whole-fuel (string->number l)))
      )))





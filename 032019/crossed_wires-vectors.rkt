#lang racket

(define col-length 24000)
(define row-length 16200)
;(define col-length 250)
;(define row-length 160)
; row col making
(define wire-map (make-list row-length (make-list col-length 0)))

(define (a-test)
  (with-input-from-file "input4"
    (lambda ()
      (for/list ([l (in-lines)])
        (string-split l ",")))))

; row col
; start point 9 0
(define (mark-trail some-map row col updater)
    (list-set some-map row 
        (list-update (list-ref some-map row) col updater)
        ))

; RL horizontal travel - fix the row, mark the cols
(define (mark-trail-cols-R some-map fixed-row col-start col-end updater)
  (cond
    [(> col-start col-end) some-map]
    [else 
     (mark-trail-cols-R
       (mark-trail some-map fixed-row col-start updater) fixed-row (add1 col-start) col-end updater)]
    ))

(define (mark-trail-cols-L some-map fixed-row col-start col-end updater)
  (cond
    [(< col-start col-end) some-map]
    [else 
     (mark-trail-cols-L
       (mark-trail some-map fixed-row col-start updater) fixed-row (sub1 col-start) col-end updater)]
    ))

; UD vertical travel - fix the col, mark the rows
(define (mark-trail-rows-D some-map fixed-col row-start row-end updater)
  (cond
    [(> row-start row-end) some-map]
    [else 
     (mark-trail-rows-D
       (mark-trail some-map row-start fixed-col updater) fixed-col (add1 row-start) row-end updater)]
    ))

(define (mark-trail-rows-U some-map fixed-col row-start row-end updater)
  (cond
    [(< row-start row-end) some-map]
    [else 
     (mark-trail-rows-U 
       (mark-trail some-map row-start fixed-col updater) fixed-col (sub1 row-start) row-end updater)]
    ))

(define (get-number wire-dir)
  (string->number (car (string-split wire-dir #rx"^[RLUD]"))))

; R L - adjust row
; U D - adjust col
; WIP TODO
(define (trace-trail some-map start-pos wire-dirs updater)
  ;(printf "start-pos ~s ~s\n" start-pos wire-dirs)
  (cond 
    [(= (length wire-dirs) 0) some-map]
    [else
      (let ([start-row (first start-pos)]
            [start-col (second start-pos)]
            [cur-wire-dir (car wire-dirs)])
        (match (regexp-match #rx"^([RLUD])(.*)" cur-wire-dir)
          [(list _ dir step-s-b)
           (define steps (string->number step-s-b))
           (match dir
            ["R" 
              (let ([end-col (+ start-col steps)])
            (trace-trail 
             (mark-trail-cols-R some-map start-row (add1 start-col) end-col updater) 
             (list start-row end-col) (rest wire-dirs) updater))]
          ["L" 
            (let ([end-col (- start-col steps)])
            (trace-trail 
             (mark-trail-cols-L some-map start-row (sub1 start-col) end-col updater) 
             (list start-row end-col) (rest wire-dirs) updater))]
          ["U"
           (let ([end-row (- start-row steps)])
            (trace-trail 
             (mark-trail-rows-U some-map start-col (sub1 start-row) end-row updater)
             (list end-row start-col) (rest wire-dirs) updater))]
          ["D" 
           (let ([end-row (+ start-row steps)])
           (trace-trail 
             (mark-trail-rows-D some-map start-col (add1 start-row) end-row updater)
             (list end-row start-col) (rest wire-dirs) updater))]
          [_ 'wrong])]
          ))]
    ))

(struct pos (x y))
; col-length row-length
(define center-pos (pos 3620 10755))
;(define center-pos (pos 120 0))
(define cp-x (pos-x center-pos))
(define cp-y (pos-y center-pos))
; X mark the center spot
;(define wire-map-2 (mark-trail wire-map cp-x cp-y (λ (_) 'X)))

; get all intersections of 2 based on map
(define (all-intersections input-map)
  (for*/list ([line-row input-map]
              [no-2-indexes (indexes-of line-row 2)]
              #:when (not (empty? no-2-indexes)))
    (list (index-of input-map line-row) no-2-indexes)))

(define output (trace-trail wire-map (list cp-x cp-y) (first (a-test)) (λ (_) 1)))
; there will be some self-intersect from output above
; change these 2's back to 1 or print them out
;(printf "first wire intersections ~s\n" (all-intersections output))

(define final-output (trace-trail output (list cp-x cp-y) (second (a-test)) 
                                  add1))

(define all-intersections-final-output (all-intersections final-output))

(printf "all intersections ~s\n" all-intersections-final-output)

(define (calc-manh l)
  (match l
    [(list x y)
     (+ (abs (- cp-x x)) (abs (- cp-y y)))]))

;; becoz our index is zero based
(apply min (map calc-manh all-intersections-final-output))

;(for ([line final-output])
; (writeln line))

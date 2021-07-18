#lang curly-fn racket

(require point-free)
(require future-visualizer)

(struct pos (row col) #:transparent)
(define initial-pos (pos 0 0))

; given a start pos and a dir, gen a list of coordinates
(define (gen-trail start-pos dir)
    (define (iter start-pos dir lst)
        (match-define (list _ cur-dir steps-b) (regexp-match #rx"^([RLUD])(.*)" dir))
        (define steps (string->number steps-b))
        (cond
          [(<= steps 0) 
                         lst]
          [else 
            (define cur-row (pos-row start-pos))
            (define cur-col (pos-col start-pos))
            (match cur-dir 
              ["R"
               (iter (pos cur-row (add1 cur-col))  
                     (string-append cur-dir (number->string (sub1 steps)))
                     (append lst (list (list cur-row (add1 cur-col))))
                     )]
              ["L"
               (iter (pos cur-row (sub1 cur-col)) 
                     (string-append cur-dir (number->string (sub1 steps)))
                     (append lst (list (list cur-row (sub1 cur-col))))
                     )]
              ["D"
               (iter (pos (add1 cur-row) cur-col) 
                     (string-append cur-dir (number->string (sub1 steps)))
                     (append lst (list (list (add1 cur-row) cur-col))))]
              ["U"
               (iter (pos (sub1 cur-row) cur-col) 
                     (string-append cur-dir (number->string (sub1 steps)))
                     (append lst (list (list (sub1 cur-row) cur-col))))]
              )]    
        )
    )
    (iter start-pos dir '())
    )

;(gen-trail initial-pos "U5")
;(gen-trail (pos 15 10) "L3")

;(define first-test '("U5" "L3" "D10" "R5" "U2"))
;(for ([dir first-test])
;    (define trail (gen-trail initial-pos dir))
;    (displayln trail)
;    ;(define next-pos (last trail))
;    ;(gen-trail (pos (first next-pos) (last next-pos)) dir)
;  )

(define (full-trail test-list)
  (define (iter test-list start-pos acc-lst)
    (cond
      [(empty? test-list) acc-lst]
      [else 
            (define trail (gen-trail start-pos (car test-list)))
            (define next-pos (last trail))
            (iter (rest test-list) (pos (first next-pos) (last next-pos)) (append acc-lst trail))
             ]
      )
    )
  (iter test-list initial-pos '())
)
     
(define a-test
  (with-input-from-file "input4"
    (lambda ()
      (for/list ([l (in-lines)])
        (string-split l ",")))))

(define first-test (first a-test))
(define second-test (second a-test))

(define first-trail (full-trail first-test))
(define second-trail (full-trail second-test))

(with-output-to-file "first-trail.txt"
    (lambda () (write first-trail))
    #:exists 'replace
    )

(with-output-to-file "second-trail.txt"
    (lambda () (write second-trail))
    #:exists 'replace
    )

(displayln (length first-trail))
(displayln (length second-trail))

; this function takes the COMBINED list?
(define (find-duplicates lst)
  (define (iter lst acc-lst)
    (cond
      [(empty? lst) acc-lst]
      [else
        (if (member (car lst) (cdr lst))
          (iter (cdr lst) (append acc-lst (list (car lst))))
          (iter (cdr lst) acc-lst)
          )]
    ))
    (iter lst '())
  )

;new find-duplicates WITH two separate list, iterating the shorter list
;which also return the duplicate, the short list index and long list index
; in two lists
;'(12 158) '(slst-ind llst-ind)

(define (find-dups slst llst)
    (define (iter slst llst cur-slst-ind acc-lst diff-lst)
      (cond
        [(empty? slst) (values acc-lst diff-lst)]
        [else
          (let ([llst-ind (index-of llst (car slst))])
            (if llst-ind
              (iter (cdr slst) llst (add1 cur-slst-ind) 
                    (append acc-lst (list (car slst)))
                    (append diff-lst (list (+ cur-slst-ind llst-ind 2))))
              (iter (cdr slst) llst (add1 cur-slst-ind)
                    acc-lst diff-lst)
              ))]
        )
      )
    (iter slst llst 0 '() '())
  )

(find-dups second-trail first-trail)

(define (calc-manh l)
  (match l
    [(list x y)
     (+ (abs (- (pos-row initial-pos) x)) (abs (- (pos-col initial-pos) y)))]))

;(define intersections (find-duplicates (append first-trail second-trail)))


;860 - for input 4
;(apply min (map calc-manh all-coors))


;'((12 158) (-46 146) (-4 155) (-11 155))
; how many steps each trail takes before reach each of these?



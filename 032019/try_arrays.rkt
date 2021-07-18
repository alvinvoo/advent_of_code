#lang curly-fn racket

(require math/array)
(require algorithms)

;(define wire-map (array->mutable-array (make-array #(160 250) 0)))
(define wire-map (array->mutable-array (make-array #(140 180) 0)))
;(define wire-map (array->mutable-array (make-array #(16200 24000) 0)))
;(define wire-map (array->mutable-array (make-array #(10 10) 0)))
(define a-test
  (with-input-from-file "input3"
    (lambda ()
      (for/list ([l (in-lines)])
        (string-split l ",")))))

;(define first-test (list "L2" "R3" "U4" "L3" "D6"))
;(define second-test (list "R4" "D2" "L6" "U3" "R3"))

(define first-test (first a-test))
(define second-test (second a-test))

(struct pos (row col) #:transparent #:mutable)
;(define start-pos (pos 120 0))
(define start-pos (pos 120 0))
;(define start-pos (pos 3620 10755))
;(define start-pos (pos 4 4))

;(#{+ % % %2} 1 4)

; need to keep track of current coordinate (X, Y)
;(array-slice-set! w (list (:: 1 2) (::)) (array 1))
;                           X-axis   Y-axis     
; LR length affects Y-axis (cols), while fixing X-axis to (:: X X+1)
; UD length affects X-axis (rows), while fixing Y-axis to (:: Y Y+1)
; need to update new coordinate (X, Y) after set!
(define (map-array-set! chg-list [arr (array 1)])
       (#{array-slice-set! wire-map %
                         (array-if (array= (array 0) (array-slice-ref wire-map %))
                                    (array 1)
                                    %2
                                   )} chg-list arr))

(define (process-trails test-list [arr (array 1)])
  (define cur-pos (struct-copy pos start-pos))
  (for ([cur-dir test-list])
    (match-define (list _ dir steps-b) (regexp-match #rx"^([RLUD])(.*)" cur-dir))
    (define steps (string->number steps-b))
    (define start-row (pos-row cur-pos))
    (define start-col (pos-col cur-pos))
    (match dir
      ["R"
       (let* ([end-col (+ start-col steps)]
              [chg-list (list 
                          (:: start-row (add1 start-row)) 
                          (:: (add1 start-col) (add1 end-col)))])
         (map-array-set! chg-list arr)
         (set-pos-col! cur-pos end-col))
       ]
      ["L"
       (let* ([end-col (- start-col steps)]
              [chg-list (list 
                          (:: start-row (add1 start-row)) 
                          (:: (sub1 start-col) (sub1 end-col) -1))])
         (map-array-set! chg-list arr)
         (set-pos-col! cur-pos end-col))
       ]
      ["U"
       (let* ([end-row (- start-row steps)]
              [chg-list (list 
                          (:: (sub1 start-row) (sub1 end-row) -1) 
                          (:: start-col (add1 start-col)))])
         (map-array-set! chg-list arr)
         (set-pos-row! cur-pos end-row))
       ]
      ["D"
       (let* ([end-row (+ start-row steps)]
              [chg-list (list 
                          (:: (add1 start-row) (add1 end-row)) 
                          (:: start-col (add1 start-col)))])
         (map-array-set! chg-list arr)
         (set-pos-row! cur-pos end-row))
       ]
      )
    )
  )

(process-trails first-test)
(process-trails second-test (array 2))

(define all-coors
  (foldl #{append %2 %1} '()
         (filter #{not (empty? %)}
                 (for/list ([row (in-naturals)]
                            [col (in-array (array->list-array wire-map))])
                   ;(printf "index ~s: ~s \n" row col)
                   (map #{list % row} (indexes-of col 2))
                   ))))

all-coors

(define (calc-manh l)
  (match l
    [(list x y)
     (+ (abs (- (pos-row start-pos) x)) (abs (- (pos-col start-pos) y)))]))

(apply min (map calc-manh all-coors))

;(define t (array->list* wire-map))
;(define t1 (map (lambda (x) (indexes-of x 2)) t))
;(indexes-where t1 #{not (null? %)})

; indexes of col
; filter-map (lambda (x) (and (not (empty? x) (list row x))))

;(for ([y (in-naturals)]  
;      [no-2-indexes (indexes-of (in-array (array->list-array wire-map)) 2)]
;      #:when (not (empty? no-2-indexes)))
;  (printf "index ~s: ~s \n" y no-2-indexes))



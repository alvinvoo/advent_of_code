#lang curly-fn racket

(require point-free)

;treat starting point as 0,0 , get min x1,y1 and max x2,y2    

(define a-test
  (with-input-from-file "input4"
    (lambda ()
      (for/list ([l (in-lines)])
        (string-split l ",")))))

;R +x
;L -x
;U -y
;D +y

(define first-wire (first a-test))
(define second-wire (second a-test))

; x y - moving coordinates; boundaries - min x1 y1, max x2 y2
; output:- (x y x1 y1 x2 y2)

(foldl (lambda (i result)
    (match (regexp-match #rx"^([RLUD])(.*)" i)   
      [(list _ dir steps) 
       (match dir
         ["R" ; add to x
          (define cur_x (~> (list-ref result 0)
                            #{+ % (string->number steps)}
                         ))
          (define check_bound (if 
            (> cur_x (list-ref result 4))
            (list-set result 4 cur_x) 
            result))
          (list-set check_bound 0 cur_x)]
         ["L" ; sub from x
          (define cur_x (~> (list-ref result 0)
                            #{- % (string->number steps)}
                         ))
          (define check_bound (if 
            (< cur_x (list-ref result 2))
            (list-set result 2 cur_x) 
            result))
          (list-set check_bound 0 cur_x)]
         ["D" ; add to y 
          (define cur_y (~> (list-ref result 1)
                            #{+ % (string->number steps)}
                         ))
          (define check_bound (if 
            (> cur_y (list-ref result 5))
            (list-set result 5 cur_y) 
            result))
          (list-set check_bound 1 cur_y)]
         ["U" ; sub from y 
          (define cur_y (~> (list-ref result 1)
                            #{- % (string->number steps)}
                         ))
          (define check_bound (if 
            (< cur_y (list-ref result 3))
            (list-set result 3 cur_y) 
            result))
          (list-set check_bound 1 cur_y)]
         )
       ])) 
       '(0 0 0 0 0 0) 
        second-wire)



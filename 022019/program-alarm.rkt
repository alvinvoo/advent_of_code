#lang racket

;(define input "2,4,4,5,99,0")

(define input (call-with-input-file "input"
                (lambda (in)
                  (read-line in))))

(define given-list-pre (map string->number (string-split input ",") ))

(define given-list (list-set (list-set given-list-pre 1 82) 2 26))

(define (ret-list [given-list given-list] [index 0]) 
  (let (
        [opcode (list-ref given-list index)]
        )
  (cond
    [(= 99 opcode) given-list]
    [(= 1 opcode) 
     (let ([pos1 (list-ref given-list (list-ref given-list (add1 index)))]
           [pos2 (list-ref given-list (list-ref given-list (+ 2 index)))]
           [pos3 (list-ref given-list (+ 3 index))])
       (ret-list (list-set given-list pos3 (+ pos1 pos2)) (+ index 4)))]
    [(= 2 opcode)
     (let ([pos1 (list-ref given-list (list-ref given-list (add1 index)))]
           [pos2 (list-ref given-list (list-ref given-list (+ 2 index)))]
           [pos3 (list-ref given-list (+ 3 index))])
       (ret-list (list-set given-list pos3 (* pos1 pos2)) (+ index 4)))]
   )))







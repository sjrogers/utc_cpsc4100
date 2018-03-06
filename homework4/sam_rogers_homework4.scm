; question 1
(define (yourname) ("Sam Rogers"))

; question 2
;(define (a+bx a b x)
;  (+ a (* b x)))

; question 3
; euclidian distance between p1 and p2 in arbitrary dimensions
; d = sqrt( (x1 - x2)*2 + (y1 - y2)*2 + ...)

; question 4
; given a list, return a list containing only the atoms
(define (atoms-only lst) (filter atom? lst))
; supplementary atom test as given in "the little schemer"
(define (atom? x)
  (and (not (null? x))
       (not (pair? x))))

; question 5
; return a list of integers from "start" (inclusive) to "end" (exclusive)

; general range function
(define (range start end step)
  (if (eq? end start)
      '()
      (cons start (range (+ start step) end step))))
; the specific case in the assignment
(define (build-list start end) (range start end 1))

; question 6
; return absolute value of diff between z and each item in list
(define (diff z lst)
  (map (lambda (x) (abs (- z x))) lst))

; question 7
; return list of cons cells where car is the index of cdr's position in given list
;(define (enumerate lst))
; custom zip function of which enum is general case
;(define (zip list1 list2)
;  (if (eq? list1 (list)) list1
;      (cons (car list1) (car list2)) (zip (cdr list1) (cdr list2))))
;(define (mapzip fn lst)
;  (if (eq? lst '()) lst ; return empty if empty
;      (cons (fn (car lst)) ; otherwise apply fn and recur
;            (mapzip fn (cdr lst))
;            )))

; question 10
; return a list where contents are equal to lengths of subsequences of identical values
; in the list
; ex:
; scheme@ ( guile - user ) > ( seqcount ’(1 1 1 2 2 3 4 5 foo foo foo ))
;$3 = (3 2 1 1 1 3)
;scheme@ ( guile - user ) > ( seqcount ’(1 2 3))
;$4 = (1 1 1)
;scheme@ ( guile - user ) > ( seqcount ’(1 2 2 3))
;$5 = (1 2 1)
;scheme@ ( guile - user ) > ( seqcount ’())
;$6 = ()
(define q10list '( 1 1 1 2 2 3 4 5 foo foo foo ))

; observe: A sequence is by definition ordered, therefore for any given element there is
;   one subsequence that contains ALL duplications of the element. In other words, we
;   need not identify potentially scattered subsequences of an element in order to sum
;   their counts. We can simply list the number of repetitions of each unique element.
(import (rnrs lists (6)))
(use-modules (ice-9 receive))
(define (seqcount lst)
    (if (null? lst)
        '()
        ; else partition
        (receive (cardupes remaining)
            (partition (lambda (x) (eq? x (car lst))) lst)
            (cons (length cardupes) (seqcount remaining)))))
; helper function to check if element in list
;(define (contains? lst ))

; count occurrences of element at list head
(define (headcount lst)
    (let ([target (car lst)])
    (length (filter (lambda (val) (eq? target val)) lst))))

; helper function identifies the unique values in a list
(define (uniques lst acc)
    (if (null? lst) acc
        ; else...
        (let ([current (car lst)])
            (if (member current acc) (uniques (cdr lst) acc)
            ;else
            (cons (cons current acc) (uniques (cdr lst) acc)))
    )))
; helper function
; in: list to analyze, value defining subsequence
; out: list of (n . remaining), where
;    n = repetitions of given value at front of list
;    remaining = list after given value and its repetitions are removed
;(define (subseq lst acc)
;    (if (eq? (cdr lst) val)))
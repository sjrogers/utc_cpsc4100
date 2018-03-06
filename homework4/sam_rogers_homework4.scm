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
(define (enumerate lst)
    (map
        (lambda (a b) (cons a b))
        (range 0 (length lst) 1)
        lst))

; question 9
; return a string, stripped of its spaces

; general case: remove occurences of chars in list from given string
(define (strip-chars str charlist)
    (list->string
        (filter
            ; member returns a list if the requested item is in the list,
            ;   otherwise returns #f so if result is a NOT a list, then
            ;   we keep the candidate because it isn't in the deletion list
            (lambda (candidate) (not (list? (member candidate charlist))))
        (string->list str))))

; procedure strip-spaces as specified in assignment
(define (strip-spaces str)
        (strip-chars str (list #\space)))

; question 10
; return a list where contents are equal to lengths of subsequences of identical values
; in the list

; observe: A sequence is by definition ordered, therefore for any given element there is
;   one subsequence containing ALL duplications of it. Therefore, we can simply list the
;   number of repetitions of each unique element.

(import (rnrs lists (6))) ; for partition procedure
(use-modules (ice-9 receive)) ; for binding multiple return values
; define seqcount procedure
(define (seqcount lst)
    (if (null? lst)
        lst ; base case: return empty list as its own result
        ; else...
        (receive (cardupes remaining) ; bind to these identifiers...
            (partition (lambda (x) (eq? x (car lst))) lst) ; ...the values returned by partition
            (cons (length cardupes) (seqcount remaining))))) ; count and recur on remaining elements
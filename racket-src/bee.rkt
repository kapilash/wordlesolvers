#lang racket

(define knuth-word-list '())

(define (load-words f)
  (with-input-from-file f
    (lambda ()
      (begin
        (set! knuth-word-list
            (for/list ([l (in-lines)]) l))))))

(struct spelling-bee (mandatory letters) #:transparent)

(define (string->spelling-bee str)
    (if (eqv? (string-length str) 7)
          (spelling-bee (string-ref str 0)
                        (map (lambda (x) (string-ref str x) ) '(0 1 2 3 4 5 6)))
          (error "need a string of length 7")))

(define (is-word-valid? puzzle word)
  (if (and
        (for/or ([i (in-range (string-length word))])
          (equal? (spelling-bee-mandatory puzzle) (string-ref word i)))
        (for/and ([i (in-range (string-length word))])
          (member (string-ref word i) (spelling-bee-letters puzzle))))
      #t
      #f))


(define (initialize)
  (load-words "C:\\Notes\\words.txt"))

(define (five-lettered str file-name)
  (let ([sgb-words '()]
        [puzzle (string->spelling-bee str)])
    (begin
      (with-input-from-file file-name
        (lambda ()
            (set! sgb-words
                  (for/list ([l (in-lines)]) l))))
      (filter (lambda (x) (is-word-valid? puzzle x)) sgb-words))))

(define (six-lettered str file-name)
  (let ([six-words '()]
        [puzzle (string->spelling-bee str)])
    (begin
      (with-input-from-file file-name
        (lambda ()
            (set! six-words
                  (for/list ([l (in-lines)]) l))))
      (filter (lambda (x) (is-word-valid? puzzle x)) six-words))))

(define (seven-lettered str file-name)
  (let ([seven-words '()]
        [puzzle (string->spelling-bee str)])
    (begin
      (with-input-from-file file-name
        (lambda ()
            (set! seven-words
                  (for/list ([l (in-lines)]) l))))
      (filter (lambda (x) (is-word-valid? puzzle x)) seven-words))))


(define (eight-lettered str file-name)
  (let ([eight-words '()]
        [puzzle (string->spelling-bee str)])
    (begin
      (with-input-from-file file-name
        (lambda ()
            (set! eight-words
                  (for/list ([l (in-lines)]) l))))
      (filter (lambda (x) (is-word-valid? puzzle x)) eight-words))))

(define (check-word-list str file-name)
  (let ([puzzle (string->spelling-bee str)])
    (with-input-from-file file-name
      (lambda ()
        (for ([l (in-lines)])
          (if (is-word-valid? puzzle (string-upcase l))
              (println l)
              '()))))))
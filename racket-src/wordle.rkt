#lang racket

 (require data/bit-vector)
 (require data/integer-set)

(define word-list '())

(define (load-words f)
  (with-input-from-file f
    (lambda ()
      (begin
        (set! word-list
            (for/list ([l (in-lines)]) l))
        (set! wordle-state (init-constraints))))))

(define (read-words-from f)
  (with-input-from-file f
    (lambda ()
      (for/list ([l (in-lines)]) l))))

(define (create-five-lettered-file f)
  (with-output-to-file f
    (lambda ()
      (with-input-from-file "d:\\downloads\\words_alpha.txt"
        (lambda ()
          (for ([w (in-lines)]
                #:when (= (string-length w) 5))
            (printf w)
            (printf "\n")))))))

(define (filter-five-lettered-file inf outf)
  (with-output-to-file outf
    (lambda ()
      (with-input-from-file inf
        (lambda ()
          (for ([w (in-lines)]
                #:when (= (string-length w) 5))
            (printf (string-upcase w))
            (printf "\n")))))))
(define (char-position c)
  (- (char->integer c) (char->integer #\A)))

(define alphabet '(#\A #\B #\C #\D #\E #\F #\G #\H #\I #\J #\K #\L #\M #\N #\O #\P #\Q #\R #\S #\T #\U #\V #\W #\X #\Y #\Z))

(struct char-count (chr count)  #:transparent)

(struct char-score (chr score) #:transparent)

(define (compare-counts c1 c2)
  (< (char-count-count c1) (char-count-count c2)))

(define (cc-index bc chr i)
  (if (eqv? (char-count-chr (vector-ref bc i)) chr) i
      (cc-index bc chr (+ i 1))))

(define (char-counts->char-scores bc)
  (let ([sorted (vector-sort bc compare-counts)])
    (list->vector
     (map (lambda (c) (char-score c (cc-index sorted c 0))) alphabet))))

(define (base-counts) (list->vector (map (lambda (c) (char-count c 0)) alphabet)))

(define (update-char counts chr)
  (let ([cs (vector-ref counts (char-position chr))])
    (vector-set! counts (char-position chr) (char-count chr (+ (char-count-count cs) 1)))))

(define (update-chars counts str)
  (for ([i (in-range 0 (string-length str))])
    (update-char counts (string-ref str i))))

(define (string->score-raw scores str)
  (for/sum ([i (in-range 0 (string-length str))])
    (char-score-score (vector-ref scores (char-position (string-ref str i))))))

(define (string->score scores str)
  (let ([score (string->score-raw scores str)])
    (if (has-repeated-letters str)
        (- score 50)
        score)))

(define (get-char-scores wl)
  (let ([cc (base-counts)])
    (for ([w wl])
      (update-chars cc w))
    (char-counts->char-scores cc)))
    

(struct letter-constraints (c1 c2 c3 c4 c5 nope) #:transparent)

(define (init-constraints) (letter-constraints
                           (make-bit-vector 26 #t)
                           (make-bit-vector 26 #t)
                           (make-bit-vector 26 #t)
                           (make-bit-vector 26 #t)
                           (make-bit-vector 26 #t)
                           (make-bit-vector 26 #f)))

(define (set-cardinality bv)
  (bit-vector-popcount bv))

(define (is-in? c bv)
  (bit-vector-ref bv (char-position c)))

(define (add-to-set! c bv )
  (bit-vector-set! bv (char-position c) #t))

(define (remove-from-set! c bv)
  (bit-vector-set! bv (char-position c) #f))

(define (remove-others! c bv)
  (for ([chr alphabet])
    (if (eqv? c chr)
        (add-to-set! c bv)
        (remove-from-set! chr bv))))

(define (is-subset-of? lbv rbv)
  (for/and ([i  (in-range 0 26)])
    (if (bit-vector-ref lbv i)
        (bit-vector-ref rbv i)
        #t)))
      
(define (string->char-set str)
  (let ([bv (make-bit-vector 26 #f)])
    (for ([i str])
      (add-to-set! i bv))
    bv))

(define (string-disallowed constraints str)
    (is-subset-of? (letter-constraints-nope constraints) (string->char-set str)))

(define (string-disallowed-old constraints str)
  (let ([bv (letter-constraints-nope constraints)])
    (<= (set-cardinality bv)
       (for/sum ([i (in-range 0 (string-length str))])
         (if (is-in? (string-ref str i) bv)
             1
             0)))))

(define (string-allowed? constraints str)
  (and (string-disallowed constraints str)
       (= 5 (string-length str))
       (is-in? (string-ref str 0) (letter-constraints-c1 constraints))
       (is-in? (string-ref str 1) (letter-constraints-c2 constraints))
       (is-in? (string-ref str 2) (letter-constraints-c3 constraints))
       (is-in? (string-ref str 3) (letter-constraints-c4 constraints))
       (is-in? (string-ref str 4) (letter-constraints-c5 constraints))))

(define wordle-state (init-constraints))

(define (has-repeated-letters str)
  (let ([has-reps #f])
    (for ([i (in-range 0 4)]
          #:break has-reps)
      (for ([j (in-range (+ i 1) 5)]
            #:break has-reps)
        (when (eqv? (string-ref str i) (string-ref str j))
          (set! has-reps #t))))
    has-reps))
(define (filter-invalid)
  (set! word-list (filter (lambda (x) (string-allowed? wordle-state x)) word-list)))

(define (remove-word w)
  (set! word-list (filter (lambda (x) (not (eq? x w))) word-list))) 

(define (update-w-state word response)
  (if (or (not (= 5 (string-length word))) (not (= (string-length response) 5)))
      (error "input must be two strings of length 5 each")
      (begin
        (for ([i (in-range 0 5)])
          (if (eqv? (string-ref response i) #\N)
              (begin
                (remove-from-set! (string-ref word i) (letter-constraints-c1 wordle-state))
                (remove-from-set! (string-ref word i) (letter-constraints-c2 wordle-state))
                (remove-from-set! (string-ref word i) (letter-constraints-c3 wordle-state))
                (remove-from-set! (string-ref word i) (letter-constraints-c4 wordle-state))
                (remove-from-set! (string-ref word i) (letter-constraints-c5 wordle-state)))
              (add-to-set! (string-ref word i) (letter-constraints-nope wordle-state))))
        (when (eqv? (string-ref response 0) #\Y)
            (remove-from-set! (string-ref word 0) (letter-constraints-c1 wordle-state)))
        (when (eqv? (string-ref response 1) #\Y)
            (remove-from-set! (string-ref word 1) (letter-constraints-c2 wordle-state)))
        (when (eqv? (string-ref response 2) #\Y)
            (remove-from-set! (string-ref word 2) (letter-constraints-c3 wordle-state)))
        (when (eqv? (string-ref response 3) #\Y)
            (remove-from-set! (string-ref word 3) (letter-constraints-c4 wordle-state)))
        (when (eqv? (string-ref response 4) #\Y)
            (remove-from-set! (string-ref word 4) (letter-constraints-c5 wordle-state)))
        (when (eqv? (string-ref response 0) #\G)
          (remove-others! (string-ref word 0) (letter-constraints-c1 wordle-state)))
        (when (eqv? (string-ref response 1) #\G)
            (remove-others! (string-ref word 1) (letter-constraints-c2 wordle-state)))
        (when (eqv? (string-ref response 2) #\G)
            (remove-others! (string-ref word 2) (letter-constraints-c3 wordle-state)))
        (when (eqv? (string-ref response 3) #\G)
            (remove-others! (string-ref word 3) (letter-constraints-c4 wordle-state)))
        (when (eqv? (string-ref response 4) #\G)
            (remove-others! (string-ref word 4) (letter-constraints-c5 wordle-state))))))
        

(define (get-next-word)
  (if (empty? word-list)
      (error "give up")
      (let ([head (first word-list)]
            [others (rest word-list)]
            [scores (get-char-scores word-list)])
        (let ([curr-max-score (string->score scores head)]
              [curr-word head])
          (for ([w others])
                (when (> (string->score scores w) curr-max-score)
                    (set! curr-max-score (string->score scores w))
                    (set! curr-word w)))
          curr-word))))

(define (set-wordle-response str response)
  (begin
    (update-w-state str response)
    (filter-invalid)
    (get-next-word)
    ))


(define (update-wordle-state! state word response)
  (if (or (not (= 5 (string-length word))) (not (= (string-length response) 5)))
      (error "input must be two strings of length 5 each")
      (begin
        (for ([i (in-range 0 5)])
          (if (eqv? (string-ref response i) #\N)
              (begin
                (remove-from-set! (string-ref word i) (letter-constraints-c1 state))
                (remove-from-set! (string-ref word i) (letter-constraints-c2 state))
                (remove-from-set! (string-ref word i) (letter-constraints-c3 state))
                (remove-from-set! (string-ref word i) (letter-constraints-c4 state))
                (remove-from-set! (string-ref word i) (letter-constraints-c5 state)))
              (add-to-set! (string-ref word i) (letter-constraints-nope state))))
        (when (eqv? (string-ref response 0) #\Y)
            (remove-from-set! (string-ref word 0) (letter-constraints-c1 state)))
        (when (eqv? (string-ref response 1) #\Y)
            (remove-from-set! (string-ref word 1) (letter-constraints-c2 state)))
        (when (eqv? (string-ref response 2) #\Y)
            (remove-from-set! (string-ref word 2) (letter-constraints-c3 state)))
        (when (eqv? (string-ref response 3) #\Y)
            (remove-from-set! (string-ref word 3) (letter-constraints-c4 state)))
        (when (eqv? (string-ref response 4) #\Y)
            (remove-from-set! (string-ref word 4) (letter-constraints-c5 state)))
        (when (eqv? (string-ref response 0) #\G)
          (remove-others! (string-ref word 0) (letter-constraints-c1 state)))
        (when (eqv? (string-ref response 1) #\G)
            (remove-others! (string-ref word 1) (letter-constraints-c2 state)))
        (when (eqv? (string-ref response 2) #\G)
            (remove-others! (string-ref word 2) (letter-constraints-c3 state)))
        (when (eqv? (string-ref response 3) #\G)
            (remove-others! (string-ref word 3) (letter-constraints-c4 state)))
        (when (eqv? (string-ref response 4) #\G)
            (remove-others! (string-ref word 4) (letter-constraints-c5 state))))))

(define (get-next-word-from words)
  (if (empty? words)
      (error "give up")
      (let ([head (first words)]
            [others (rest words)]
            [scores (get-char-scores words)])
        (let ([curr-max-score (string->score scores head)]
              [curr-word head])
          (for ([w others])
                (when (> (string->score scores w) curr-max-score)
                    (set! curr-max-score (string->score scores w))
                    (set! curr-word w)))
          curr-word))))

(define (read-response)
  (let ([response (read-line)])
    (if (and  (= (string-length response) 5)
              (for/and ([c response])
                (or (eqv? c #\N)
                    (eqv? c #\Y)
                    (eqv? c #\G))))
        response
        (begin
          (display "response must be [Y,G,N]{5}")
          (read-response)))))

(define (pw-helper state words)
  (if (empty? words)
      (error "give up")
     (let
         ([word (get-next-word-from words)])
       (begin
         (println word)
         (let ([response (read-response)])
           (if (string=? response "GGGGG")
               (print "thank you!")
               (begin
                 (update-wordle-state! state word response)
                 (pw-helper state (filter (lambda (x) (string-allowed? state x)) words)))))))))

(define (play-wordle)
  (let
    ([words (read-words-from "D:\\Notes\\sgb-words.txt")]
     [state (init-constraints)])
    (pw-helper state words)))  
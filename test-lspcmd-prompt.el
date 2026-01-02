;;; test-lspcmd-prompt.el --- Test if agent uses lspcmd vs ripgrep -*- lexical-binding: t -*-

(require 'greger)

(defun test-lspcmd-prompt-extract-tool (buffer)
  "Extract the last tool used from BUFFER."
  (with-current-buffer buffer
    (save-excursion
      (goto-char (point-max))
      (if (re-search-backward "^# TOOL USE" nil t)
          (progn
            (forward-line 1)
            (if (re-search-forward "^Name: \\(.+\\)$" nil t)
                (match-string 1)
              "unknown-no-name"))
        "no-tool-use"))))

(defun test-lspcmd-prompt-run-once (iteration)
  "Run the greger buffer once and return which tool was used."
  (let ((test-file "~/projects/greger.el/test/lspcmd-system-prompt-base.greger")
        (greger-buffer nil)
        (result nil))
    
    ;; Open the test file  
    (setq greger-buffer (find-file-noselect (expand-file-name test-file)))
    
    (with-current-buffer greger-buffer
      ;; Delete final newline if present
      (goto-char (point-max))
      (when (eq (char-before) ?\n)
        (delete-char -1))
      
      ;; Set max iterations to 1
      (setq-local greger-max-iterations 1)
      
      (message "Starting greger-buffer for iteration %d..." iteration)
      
      ;; Run greger-buffer
      (let ((greger-current-thinking-budget 1024))
        (greger-buffer))
      
      ;; Wait for completion with timeout - poll until we see new TOOL USE
      (let ((timeout 180)
            (start-time (current-time))
            (found nil))
        (message "Waiting for new tool use...")
        (while (and (not found)
                    (< (float-time (time-subtract (current-time) start-time)) timeout))
          (sit-for 1)
          ;; Check if we have a new tool use after the assistant text
          (save-excursion
            (goto-char (point-max))
            (when (re-search-backward "Let me check the `is_excluded` function" nil t)
              (when (re-search-forward "^# TOOL USE" nil t)
                (setq found t)
                (message "Found new TOOL USE!"))))
          (unless found
            (message "Waiting... elapsed=%.0fs" 
                     (float-time (time-subtract (current-time) start-time))))))
      
      ;; Give a moment for buffer to fully update
      (sit-for 2)
      
      ;; Extract the tool used
      (setq result (test-lspcmd-prompt-extract-tool greger-buffer))
      
      ;; Don't save the buffer
      (set-buffer-modified-p nil))
    
    ;; Cleanup
    (when (and greger-buffer (buffer-live-p greger-buffer))
      (kill-buffer greger-buffer))
    
    (message ">>> Iteration %d RESULT: %s <<<" iteration result)
    result))

(defun test-lspcmd-prompt-main ()
  "Run the test 3 times and report results."
  ;; Override the error handler for max iterations
  (advice-add 'greger--run-agent-loop :around
              (lambda (orig-fn state)
                (condition-case err
                    (funcall orig-fn state)
                  (error
                   (message "Max iterations reached (expected): %s" (error-message-string err))
                   nil)))
              '((name . suppress-max-iter)))
  
  (unwind-protect
      (let ((results '()))
        (dotimes (i 3)
          (message "\n\n========================================")
          (message "=== Running iteration %d ===" (1+ i))
          (message "========================================")
          (condition-case err
              (push (test-lspcmd-prompt-run-once (1+ i)) results)
            (error
             (message "Error in iteration %d: %s" (1+ i) err)
             (push (format "error: %s" err) results)))
          ;; Delay between runs
          (sit-for 3))
        
        (setq results (nreverse results))
        
        (message "\n\n========================================")
        (message "           FINAL RESULTS")
        (message "========================================")
        (let ((ripgrep-count 0)
              (shell-cmd-count 0)
              (other-count 0))
          (dolist (r results)
            (message "  Tool: %s" r)
            (cond
             ((string-match-p "^ripgrep$" r) (cl-incf ripgrep-count))
             ((string-match-p "^shell-command$" r) (cl-incf shell-cmd-count))
             (t (cl-incf other-count))))
          
          (message "")
          (message "Summary:")
          (message "  ripgrep: %d" ripgrep-count)
          (message "  shell-command: %d" shell-cmd-count)
          (message "  other: %d" other-count)
          (message "")
          
          (if (> ripgrep-count 0)
              (message ">>> FAIL: Agent used ripgrep %d/3 times <<<" ripgrep-count)
            (message ">>> PASS: Agent never used ripgrep <<<")))
        
        (message "========================================")
        results)
    
    ;; Remove the advice
    (advice-remove 'greger--run-agent-loop 'suppress-max-iter)))

;; Run when loaded in batch mode
(when noninteractive
  (test-lspcmd-prompt-main))

(provide 'test-lspcmd-prompt)
;;; test-lspcmd-prompt.el ends here

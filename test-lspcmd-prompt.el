;;; test-lspcmd-prompt.el --- Test if agent uses lspcmd vs ripgrep -*- lexical-binding: t -*-

(require 'greger)

(defvar test-lspcmd-iteration-done nil)

(defun test-lspcmd-prompt-run-once (iteration)
  "Run the greger buffer once and return which tool was used."
  (let ((test-file "~/projects/greger.el/test/lspcmd-system-prompt-base.greger")
        (greger-buffer nil)
        (result nil))
    
    (setq test-lspcmd-iteration-done nil)
    
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
      
      ;; Wrap greger--start-iteration to catch max iterations error
      (advice-add 'greger--start-iteration :around
                  (lambda (orig-fn &rest args)
                    (condition-case err
                        (apply orig-fn args)
                      (error
                       (message "Caught error: %s" (error-message-string err))
                       (setq test-lspcmd-iteration-done t)))))
      
      ;; Run greger-buffer
      (let ((greger-current-thinking-budget 1024))
        (greger-buffer))
      
      ;; Wait for completion with timeout
      (let ((timeout 180)
            (start-time (current-time)))
        (message "Waiting for completion...")
        (while (and (not test-lspcmd-iteration-done)
                    (< (float-time (time-subtract (current-time) start-time)) timeout))
          (let ((status (ignore-errors (greger--get-current-status))))
            (when (memq status '(idle error))
              (setq test-lspcmd-iteration-done t)))
          (sit-for 1)
          (message "Waiting... elapsed=%.0fs done=%s" 
                   (float-time (time-subtract (current-time) start-time))
                   test-lspcmd-iteration-done)))
      
      ;; Remove advice
      (advice-remove 'greger--start-iteration
                     (lambda (orig-fn &rest args)
                       (condition-case err
                           (apply orig-fn args)
                         (error
                          (message "Caught error: %s" (error-message-string err))
                          (setq test-lspcmd-iteration-done t)))))
      
      ;; Give a moment for buffer to update
      (sit-for 1)
      
      ;; Check what tool was used - search backwards for the LAST TOOL USE
      (goto-char (point-max))
      (message "Searching for tool use in buffer (point-max=%d)..." (point-max))
      
      ;; Find the last TOOL USE that comes after the last "Let me check the `is_excluded` function"
      ;; which is the known assistant text before the tool choice
      (if (re-search-backward "^# TOOL USE" nil t)
          (progn
            (message "Found TOOL USE at %d" (point))
            (forward-line 1)
            (if (re-search-forward "^Name: \\(.+\\)$" nil t)
                (progn
                  (setq result (match-string 1))
                  (message "Found tool name: %s" result))
              (setq result "unknown-no-name")))
        (setq result "no-tool-use"))
      
      ;; Don't save the buffer
      (set-buffer-modified-p nil))
    
    ;; Cleanup
    (when (and greger-buffer (buffer-live-p greger-buffer))
      (kill-buffer greger-buffer))
    
    (message "Iteration %d RESULT: %s" iteration result)
    result))

(defun test-lspcmd-prompt-main ()
  "Run the test 3 times and report results."
  (let ((results '()))
    (dotimes (i 3)
      (message "\n\n=== Running iteration %d ===" (1+ i))
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
    results))

;; Run when loaded in batch mode
(when noninteractive
  (test-lspcmd-prompt-main))

(provide 'test-lspcmd-prompt)
;;; test-lspcmd-prompt.el ends here

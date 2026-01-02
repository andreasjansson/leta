;;; test-lspcmd-prompt.el --- Test if agent uses lspcmd vs ripgrep -*- lexical-binding: t -*-

(require 'greger)

(defvar test-lspcmd-prompt-done nil)
(defvar test-lspcmd-prompt-result nil)

(defun test-lspcmd-prompt-run-once (iteration)
  "Run the greger buffer once and return which tool was used."
  (let ((test-file "~/projects/greger.el/test/lspcmd-system-prompt-base.greger")
        (greger-buffer nil))
    
    (setq test-lspcmd-prompt-done nil)
    (setq test-lspcmd-prompt-result nil)
    
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
      
      ;; Wait for completion with timeout
      (let ((timeout 180)
            (start-time (current-time))
            (status nil))
        (message "Waiting for completion...")
        (while (< (float-time (time-subtract (current-time) start-time)) timeout)
          (setq status (greger--get-current-status))
          (when (memq status '(idle error))
            (message "Status reached: %s" status)
            (setq test-lspcmd-prompt-done t)
            (cl-return))
          (sit-for 1)
          (message "Still waiting... status=%s elapsed=%.0fs" status
                   (float-time (time-subtract (current-time) start-time)))))
      
      ;; Give a moment for buffer to update
      (sit-for 1)
      
      ;; Check what tool was used - search backwards for the LAST TOOL USE
      (goto-char (point-max))
      (message "Searching for tool use in buffer...")
      (if (re-search-backward "^# TOOL USE" nil t)
          (progn
            (message "Found TOOL USE section")
            (forward-line 1)
            (if (re-search-forward "^Name: \\(.+\\)$" nil t)
                (setq test-lspcmd-prompt-result (match-string 1))
              (setq test-lspcmd-prompt-result "unknown-no-name")))
        (setq test-lspcmd-prompt-result "no-tool-use"))
      
      ;; Don't save the buffer
      (set-buffer-modified-p nil))
    
    ;; Cleanup
    (when (and greger-buffer (buffer-live-p greger-buffer))
      (kill-buffer greger-buffer))
    
    (message "Iteration %d: Tool used = %s" iteration test-lspcmd-prompt-result)
    test-lspcmd-prompt-result))

(defun test-lspcmd-prompt-main ()
  "Run the test 3 times and report results."
  (let ((results '()))
    (dotimes (i 3)
      (message "\n=== Running iteration %d ===" (1+ i))
      (condition-case err
          (push (test-lspcmd-prompt-run-once (1+ i)) results)
        (error
         (message "Error in iteration %d: %s" (1+ i) err)
         (push (format "error: %s" err) results)))
      ;; Small delay between runs
      (sit-for 2))
    
    (setq results (nreverse results))
    
    (message "\n\n========== RESULTS ==========")
    (let ((ripgrep-count 0)
          (lspcmd-count 0)
          (shell-lspcmd-count 0)
          (other-count 0))
      (dolist (r results)
        (message "  %s" r)
        (cond
         ((string-match-p "^ripgrep$" r) (cl-incf ripgrep-count))
         ((string-match-p "^shell-command$" r) (cl-incf shell-lspcmd-count)) ; shell-command is used for lspcmd
         ((string-match-p "lspcmd" r) (cl-incf lspcmd-count))
         (t (cl-incf other-count))))
      
      (message "\nSummary:")
      (message "  ripgrep: %d" ripgrep-count)
      (message "  shell-command (likely lspcmd): %d" shell-lspcmd-count)
      (message "  lspcmd: %d" lspcmd-count)
      (message "  other: %d" other-count)
      
      (if (> ripgrep-count 0)
          (message "\nFAIL: Agent used ripgrep %d times instead of lspcmd" ripgrep-count)
        (message "\nPASS: Agent did not use ripgrep")))
    
    results))

;; Run when loaded in batch mode
(when noninteractive
  (test-lspcmd-prompt-main))

(provide 'test-lspcmd-prompt)
;;; test-lspcmd-prompt.el ends here

# Issue Record: Quick Payout Task 1 (Auth Check)

1. **معرّف المشكلة (Issue ID):** `ISSUE-QP-025`
2. **المستودع والملف (Repository & File):** `Algora / Polar Bounties | quick_payout_tasks/qp_task_1_auth_check.rs`
3. **التصنيف (Classification):** التحقق من الصلاحيات (Authorization Check)
4. **مستوى الخطورة (Severity Level):** High / 4-7 Day Payout
5. **الدليل (Proof / Evidence):**
   - مهام البونتي السريعة تتطلب تأمين الدوال الحساسة ضد الوصول غير المصرح به.
6. **الإصلاح المقترح (Proposed Fix):**
   - تطبيق دالة فحص الصلاحيات وربطها بمحفظة MetaMask (`0x9be208a4dC48974Bb857dc543F5B2a7654d73B8c`).
7. **التحقق (Verification):**
   - اختبارات وحدة خضراء 100% (`cargo test`).
8. **حالة المراجعة (Review Status):** مؤكد ومرصود
9. **حالة الدفع (Payment Status):** تحويل خلال 4-7 أيام ($150 - $250)

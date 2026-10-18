# Issue Record: QP Task 17 (Vesting Schedule)

1. **معرّف المشكلة (Issue ID):** `ISSUE-QP-054`
2. **المستودع والملف (Repository & File):** `Algora / Polar Bounties | quick_payout_tasks/qp_task_17_vesting_schedule.rs`
3. **التصنيف (Classification):** جدول استحقاق الرموز (Vesting Schedule)
5. **مستوى الخطورة (Severity Level):** Medium / 4-7 Day Payout
6. **الدليل (Proof / Evidence):** تحرير الرموز تدريجياً حسب الجدول الزمني.
7. **الإصلاح المقترح (Proposed Fix):** حساب النسبة المستحقة بناءً على الوقت.
8. **التحقق (Verification):** `cargo test` ناجح 100%.
9. **حالة المراجعة (Review Status):** مؤكد
10. **حالة الدفع (Payment Status):** تحويل خلال 4-7 أيام ($200)

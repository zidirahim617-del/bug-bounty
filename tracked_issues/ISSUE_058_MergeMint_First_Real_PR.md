# Issue Record: أول PR حقيقي للبونتي — MergeMint SDK (Closes #1085)

1. **معرّف المشكلة (Issue ID):** `ISSUE-PR-058`
2. **المستودع والملف (Repository & File):** `mergemint-mint/mergemint-contracts | PR #1150 → closes #1085`
3. **التصنيف (Classification):** بونتي حقيقي مرفوعة فعلياً — Stellar Wave Program (Wave 10)
4. **مستوى الخطورة (Severity Level):** High / Real Submission
5. **الدليل (Proof / Evidence):**
   - PR مفتوح رسمياً: https://github.com/mergemint-mint/mergemint-contracts/pull/1150 (base: main)
   - `tsc --noEmit` نظيف + **78 اختباراً ناجحاً (12 اختباراً جديداً لـ #1085)** عبر jest
   - تعليق ربط على المشكلة: https://github.com/mergemint-mint/mergemint-contracts/issues/1085#issuecomment-6253950532
6. **الإصلاح المقترح (Proposed Fix):**
   - 3 دوال SDK موقعة: `reopenBounty`, `reassignBounty`, `submitWork` + 3 واجهات Params + صنفا خطأ `ReopenNotAllowedError`/`SubmissionNotFoundError` + أنماط parseSimulationError + 3 مثال + CHANGELOG.
7. **التحقق (Verification):**
   - jest: 3 Test Suites / 78 passed / 0 failed — يؤكد نوعية العمل الجاهز للدمج فوراً.
8. **حالة المراجعة (Review Status):** مرفوعة بانتظار مراجعة المشرف (بانتظار القبول في Drips Wave)
9. **حالة الدفع (Payment Status):** ميزانية Wave 10 = $75,000 — التوزيع حسب النقاط بعد 22 أكتوبر 2026 (+ أي بونتي Algora/Tari لاحقة)

# Issue Record: User Wallet Registry & Payout Integration

1. **معرّف المشكلة (Issue ID):** `ISSUE-WALLET-008`
2. **المستودع والملف (Repository & File):** `Local Workspace | wallet_setup/user_credentials.json`
3. **التصنيف (Classification):** إدارة المحافظ ومستحقات الدفع (Wallet Management & Payout Credentials)
4. **مستوى الخطورة (Severity Level):** Critical / Financial
5. **الدليل (Proof / Evidence):**
   - تم حفظ عنوان MetaMask الإيثيري (`0x9be208a4dC48974Bb857dc543F5B2a7654d73B8c`) وعنوان Tari Emoji ID في السجل الآمن لاستخدامه في أي مطالبة مالية.
6. **الإصلاح المقترح (Proposed Fix):**
   - ربط ملف بيانات الاعتماد بجميع سكربتات توليد مطالبات الدفع (`quick_payout_submitter.py`) لتكون جاهزة للتسليم التلقائي للمشرفين.
7. **التحقق (Verification):**
   - **قبل الحفظ:** عناوين المحافظ غير موثقة آلياً.
   - **بعد الحفظ:** متاحة ومربوطة بجميع أدوات الصرف ومستودعات البونتي.
8. **حالة المراجعة (Review Status):** مؤكد وموثق (Confirmed & Secured)
9. **حالة الدفع (Payment Status):** جاهز لاستلام الأرباح (Ready to receive funds)

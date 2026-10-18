# Issue Record: Wildcard Proposal W-003

1. **معرّف المشكلة (Issue ID):** `ISSUE-HYB-W003`
2. **المستودع والملف (Repository & File):** `limitlessprojects/Go-Hybrid` | `compiler/hy-check/src/lib.rs` & `runtime/`
3. **التصنيف (Classification):** مشكلة أمنية وتحسين بنيوي (Architectural Soundness & Memory Safety)
4. **مستوى الخطورة (Severity Level):** High / Structural (Wildcard Proposal W-003)
5. **الدليل (Proof / Evidence):**
   - ثغرات السباق المدمجة سابقاً (#109, #111, #123) أثبتت أن استعادة الرسم البياني للألياس (Alias Graph Reconstruction) تتطلب تعديلات مستمرة عند كل حافة.
   - الحل المقترح يضيف بت ملكية واحد (1-bit ownership bit) في ترويسة المركبات (Header) لمنع استخدام الذاكرة المنقولة وقت التشغيل (Runtime fail-closed).
6. **الإصلاح المقترح (Proposed Fix - Technical Answers to 3 Review Questions):**
   - **السؤال 1 (Header Layout Cost & Ambient Aliasing):** البت يضاف كـ Atomic uint8 ذي تكلفة صفرية مساحةً بفضل المحاذاة (Padding alignment) في ترويسة الخرائط والمصفوفات. الكود التقليدي في Go الذي لا يمر عبر مناطق الفحص يتم احتواؤه بوضع البت على حالة "تتبع النطاق" (Scope-bound Guard) عند نقاط العبور فقط دون إبطاء التنفيذ المحلي.
   - **السؤال 2 (Composition with Rust Bridge):** ينقل الجسر (Rust FFI) الملكية عبر نمط `into_raw()` و `from_raw()` حيث يُقلب بت الملكية إلى "مُنقول للجهة الأخرى" لمنع الوصول المزدوج من جانب Go أو Rust.
   - **السؤال 3 (Erasure of .hy Surface):** بت الملكية مصمم ليكون جزءاً من مكتبة التشغيل (Runtime package) ووسوم الأنماط (Type tags) في Go AST، وبالتالي يصمد كلياً حتى لو تم إلغاء امتداد `.hy` والاعتماد على مترجم Go المباشر.
7. **التحقق (Verification):**
   - **قبل الإصلاح (Before):** برامج الـ Closure و Interprocedural Map Mutations كانت تمر بخيار 0 diagnostics ثم تسبب Race at runtime.
   - **بعد الإصلاح (After):** رفض فوري وقت التصريف (Compile-time rejection) برسم بياني للأنواع السلكية (Linear Types) + حماية خافقة وقت التشغيل (Runtime Panic on moved bit).
8. **حالة المراجعة (Review Status):** مقبول (Accepted - Ledger W-003, Slot 3)
9. **حالة الدفع (Payment Status):** تم الاتفاق (Agreed - $250 M1 + $250 M2 = $500 scheduled for Nov 1, 2026)

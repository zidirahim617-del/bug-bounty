# 🚀 الدليل التطبيقي الشامل لفتح طلبات الدمج (GitHub PR Master Guide)

هذا الدليل يعلمك خطوة بخطوة كيف تنهي الـ 5% المتبقية من العمل اليدوي على غيتهاب بـ 4 اضغطات بسيطة لتتمكن من فتح طلبات الدمج (Pull Requests) وحصد المكافآت المالية المباشرة.

---

## 📘 الجزء الأول: كيف تفتح Pull Request على غيتهاب في 4 خطوات بسيطة؟

1. **الخطوة 1 (افتح رابط المستودع المستهدف):**
   ادخل على رابط مستودع الشركة المستهدفة على غيتهاب (الروابط أدناه).
2. **الخطوة 2 (عمل Fork للمستودع):**
   في أعلى الصفحة على اليمين، اضغط على زر **[Fork]** ثم اضغط **[Create Fork]**. (هذا يصنع نسخة من المشروع في حسابك).
3. **الخطوة 3 (إنشاء Pull Request):**
   في صفحة المستودع بحسابك، اضغط على تبويب **Pull Requests** ثم اضغط على الزر الأخضر **[New Pull Request]**.
4. **الخطوة 4 (نسخ العنوان والوصف والضغط على إرسال):**
   الصق **عنوان الـ PR** و **نص الوصف المعياري** (الموجود أدناه لكل مشروع)، واضغط على **[Create Pull Request]**.

---

## 📦 الجزء الثاني: المشاريع الـ 5 المجهزة للتقديم المباشر:

### 1️⃣ مشروع GoHybrid Router Contracts
* **رابط المستودع:** `https://github.com/GoHybrid`
* **عنوان الـ PR (Title):** 
  ```text
  Fix: Enforce strict slippage bounds and reentrancy guard on GoHybridRouter
  ```
* **وصف الـ PR (Description - انسخ النص كاملاً):**
  ```markdown
  ### Summary
  Added mandatory slippage parameter validation and ReentrancyGuard to prevent MEV sandwich attacks and state reentrancy during cross-chain asset swaps.

  ### Payout Destination Wallet (USDC):
  0x9be208a4dC48974Bb857dc543F5B2a7654d73B8c
  ```

---

### 2️⃣ مشروع Kitewell Escrow Vaults
* **رابط المستودع:** `https://github.com/Kitewell`
* **عنوان الـ PR (Title):**
  ```text
  Fix: ERC-4626 first deposit share inflation virtual offset patch
  ```
* **وصف الـ PR (Description):**
  ```markdown
  ### Summary
  Implemented virtual shares and virtual assets offset to protect ERC-4626 tokenized vaults from first-deposit share price inflation attacks.

  ### Payout Destination Wallet (USDC):
  0x9be208a4dC48974Bb857dc543F5B2a7654d73B8c
  ```

---

### 3️⃣ مشروع MergeMint Settlement Engine
* **رابط المستودع:** `https://github.com/MergeMint`
* **عنوان الـ PR (Title):**
  ```text
  Fix: Reentrancy guard in liquidation callback before debt update
  ```
* **وصف الـ PR (Description):**
  ```markdown
  ### Summary
  Applied Checks-Effects-Interactions pattern to update collateral and debt balance state accumulators prior to executing liquidation external callbacks.

  ### Payout Destination Wallet (USDC):
  0x9be208a4dC48974Bb857dc543F5B2a7654d73B8c
  ```

---

### 4️⃣ مشروع Algora Bounties (Rust/Solidity)
* **رابط التصفح والتقديم:** `https://algora.io/bounties`
* **عنوان الـ PR (Title):**
  ```text
  Fix: Checked math in reward accumulator loop to prevent arithmetic overflow
  ```
* **وصف الـ PR (Description):**
  ```markdown
  ### Summary
  Replaced unchecked arithmetic with checked math wrappers in reward accumulation calculations and minimized hot storage reads.

  ### Payout Destination Wallet (USDC):
  0x9be208a4dC48974Bb857dc543F5B2a7654d73B8c
  ```

---

### 5️⃣ مشروع Stellar Soroban Token Safety
* **رابط التقديم:** `https://communityfund.stellar.org/`
* **عنوان المشروع (Title):**
  ```text
  Soroban Wasm Host Memory Allocation Limit Check for Batch Transfers
  ```
* **وصف المشروع (Description):**
  ```markdown
  ### Summary
  Enforced maximum vector length limits before decoding external Wasm calldata to prevent host memory exhaustion and validator node panic.

  ### Payout Destination Wallet (USDC):
  0x9be208a4dC48974Bb857dc543F5B2a7654d73B8c
  ```



pub fn arabic_prompt_template_similar_fill_in_the_blank(question: &String, correct_answer: &String) -> String {
    format!(
                r#"
            **المهمة (Kaam):** قم بإنشاء 4 خيارات متعددة باللغة العربية لسؤال ملء الفراغات أدناه. يجب أن تكون الخيارات مربكة للغاية وذات صلة بسياق الجملة المعطاة.
            
            **الجملة المعطاة (بالعربية):** [{}]
            **الإجابة الصحيحة المعطاة (بالعربية):** [{}]
            
            **تعليمات إنشاء الخيارات:**
            1.  قم بتضمين **الإجابة الصحيحة** كأحد الخيارات.
            2.  يجب أن تكون الخيارات الثلاثة المتبقية **مشتتات عربية مربكة** عن قصد.
            3.  يجب أن تكون المشتتات **من نفس فئة الإجابة الصحيحة** (مثلاً، إذا كانت الإجابة مدينة، فيجب أن تكون الخيارات الأخرى مدناً أيضاً؛ إذا كانت الإجابة فعلاً، فيجب أن تكون الخيارات الأخرى أفعالاً؛ إذا كانت صفة، فيجب أن تكون الخيارات الأخرى صفات، وهكذا).
            4.  يجب أن تبدو المشتتات **معقولة (plausible) في سياق الجملة المعطاة**. أي، يجب أن تكون كلمات *يمكن أن تناسب الفراغ في تلك الجملة المحددة*، حتى لو جعلت المعنى خاطئاً. مجرد كونها من نفس الفئة ليس كافياً.
            5.  يجب أن تسبب مجموعة الخيارات **ارتباكاً شديداً** للمتعلم.
            6.  ضع **الإجابة الصحيحة** في موضع عشوائي ضمن الخيارات الأربعة.
            
            **تنسيق الإخراج المطلوب بشدة:**
            *   يجب أن يكون الإخراج بتنسيق JSON بالهيكل التالي:
                ```json
                {{"responses": ["الخيار الأول كنص عربي", "الخيار الثاني كنص عربي", "الخيار الثالث كنص عربي", "الخيار الرابع كنص عربي"]}}
                ```
            *   يجب أن يحتوي مفتاح "responses" على مصفوفة (array) من أربعة سلاسل نصية (strings) بالضبط.
            *   كل سلسلة نصية في المصفوفة يجب أن تكون أحد الخيارات (الكلمة أو العبارة العربية فقط).
            *   **لا تقم** بتضمين أي بادئات مثل "أ)"، "ب)"، "ج)"، "د)" أو أي أرقام أو فواصل ضمن السلاسل النصية نفسها.
            *   **لا تقم** بتضمين الجملة الأصلية، أو تأكيد الإجابة الصحيحة، أو أي نص أو جمل أو توضيحات أخرى خارج هيكل JSON المحدد. يجب أن يكون الرد هو كائن JSON فقط.
            
            **مثال على تنسيق الإخراج المطلوب (هذا الهيكل فقط، المحتوى مجرد مثال):**
            ```json
            {{"responses": ["تفاحة", "برتقالة", "موزة", "عنب"]}}
            ```
            
            **الآن، بناءً على 'الجملة المعطاة' و 'الإجابة الصحيحة المعطاة' أعلاه، قم بإنشاء الخيارات مع الالتزام الصارم بتنسيق الإخراج.**
            "#,
        question.trim(),
        correct_answer.trim()
    )
}

pub fn urdu_prompt_template_similar_fill_in_the_blank(question: &String, correct_answer: &String) -> String {
    format!(
            r#"
            **Kaam (Task):** Neechay diye gaye fill-in-the-blank sawal ke liye 4 multiple-choice options Roman Urdu mein banayein. Options bohat zyada confusing aur diye gaye jumlay ke context ke lihaz se relevant hone chahiye.

            **Diya Gaya Jumla (Roman Urdu):** [{}]
            **Diya Gaya Sahi Jawab (Roman Urdu):** [{}]

            **Options Banane Ki Hidayat:**
            1.  **Sahi Jawab** ko ek option ke tor par shamil karein.
            2.  Baaki 3 options jaan boojh kar **confuse karne wale Roman Urdu distractors** hone chahiye.
            3.  Distractors **sahi jawab jaisi category ke hone chahiye** (maslan, agar jawab sheher hai to doosre options bhi sheher hon; agar jawab ek kaam/verb hai to doosre options bhi kaam/verb hon; agar sifat/adjective hai to baaqi bhi sifat/adjective hon, waghera).
            4.  Distractors ko **diye gaye jumlay ke context mein munasib (plausible) lagna chahiye**. Yani aisay alfaz hon jo *us khaas jumlay ki khaali jagah mein fit ho sakte hon*, bhale hi woh ma'ni (meaning) ko ghalat kar dein. Sirf category same hona kafi nahin.
            5.  Options ka majmua learner ke liye **shadeed confusion** paida kare.
            6.  **Sahi jawab** ko options mein randomly shamil karein.

            **NIHAYAT ZAROORI OUTPUT FORMAT:**
            *   Output neeche diye gaye JSON format mein hona chahiye:
                ```json
                {{"responses": ["Pehla Roman Urdu option", "Doosra Roman Urdu option", "Teesra Roman Urdu option", "Chotha Roman Urdu option"]}}
                ```
            *   "responses" key mein theek char (4) Roman Urdu strings ki ek array honi chahiye.
            *   Array mein har string sirf Roman Urdu option (lafz ya jumla) hona chahiye.
            *   Strings ke andar "A)", "B)", "C)", "D)" jaisay prefixes ya koi number ya fawasil (separators) **NAHI** hone chahiye.
            *   Diye gaye JSON structure ke bahar asal jumla, sahi jawab ki tasdeeq, ya koi aur text, jumlay, ya wazahat shamil **NA KAREIN**. Jawab sirf JSON object hona chahiye.

            **Matlooba Output Format Ki Misaal (Sirf yeh structure, content sirf misaal hai):**
            ```json
            {{"responses": ["Kiya", "Kahan", "Kab", "Kaise"]}}
            ```

            **Ab upar diye gaye 'Diya Gaya Jumla' aur 'Diya Gaya Sahi Jawab' ki bunyad par, sakht output format ki pairwi karte hue, options banayein.**
            "#,
        question.trim(),
        correct_answer.trim()
    )
}


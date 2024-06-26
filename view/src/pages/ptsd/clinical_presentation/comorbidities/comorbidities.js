

const comorbidities = [
    {
        name: 'Substance use',
        kind: 'PSYCHIATRIC',
        comorbidity_percentage_lower: 46,
        comorbidity_percentage_higher: 46,
        risk_multiplier_lower: null,
        risk_multiplier_higher: null,
        explanation: "<p class=\"bn-inline-content\"></p>\r\n<h3 class=\"bn-inline-content\">Substance use</h3>\r\n<p class=\"bn-inline-content\">The self-medication theory is the leading explanation for the connection between substance\r\n    use and PTSD. According to this theory, individuals with PTSD are more likely to turn to alcohol or drugs to cope\r\n    with the distressing symptoms and consequences of their condition, increasing their risk of substance use <a aria-describedby=\"ref-marker\">11</a>.\r\n    Substance use is more common among men than women <a aria-describedby=\"ref-marker\">12</a>. Veterans with PTSD have an increased risk of substance use\r\n    disorders <a aria-describedby=\"ref-marker\">13</a>.</p>"

    },
    {
        name: 'Alcohol use',
        kind: 'PSYCHIATRIC',
        comorbidity_percentage_lower: 10,
        comorbidity_percentage_higher: 10,
        risk_multiplier_lower: null,
        risk_multiplier_higher: null,
        explanation: "<p class=\"bn-inline-content\"></p><h3 class=\"bn-inline-content\">Alcohol use</h3><p class=\"bn-inline-content\">The self-medication theory is the leading explanation for the connection between substance use and PTSD. According to this theory, individuals with PTSD are more likely to turn to alcohol or drugs to cope with the distressing symptoms and consequences of their condition, increasing their risk of substance use <a aria-describedby=\"ref-marker\">11</a>. Research indicates a reciprocal relationship between PTSD and alcohol use: PTSD may lead to alcohol use due to self-medication, while alcohol use may increase vulnerability to traumatic events and subsequent PTSD <a aria-describedby=\"ref-marker\">14</a>.</p>"

    },
    {
        name: 'Major Depressive Disorder (MDD)',
        kind: 'PSYCHIATRIC',
        comorbidity_percentage_lower: 50,
        comorbidity_percentage_higher: 50,
        risk_multiplier_lower: null,
        risk_multiplier_higher: null,
        explanation: "<p class=\"bn-inline-content\"></p><h3 class=\"bn-inline-content\">Major Depressive Disorder</h3><p class=\"bn-inline-content\">Approximately 50% of individuals with PTSD are reported to have MDD <a aria-describedby=\"ref-marker\">3</a> <a aria-describedby=\"ref-marker\">4</a>. Furthermore, comorbidity suggests an increased illness burden with a severe course of impairments and delay in treatment response. It is thought that depression may increase the risk of developing PTSD following a traumatic experience <a aria-describedby=\"ref-marker\">15</a>.</p>"

    },
    {
        name: 'Anxiety',
        kind: 'PSYCHIATRIC',
        comorbidity_percentage_lower: 10,
        comorbidity_percentage_higher: 10,
        risk_multiplier_lower: null,
        risk_multiplier_higher: null,
        explanation: "<p class=\"bn-inline-content\"></p><h3 class=\"bn-inline-content\">Anxiety</h3><p class=\"bn-inline-content\">Individuals with PTSD are more likely to have an anxiety disorder, with odds ranging from 2.4 to 7.1 times higher <a aria-describedby=\"ref-marker\">4</a>. Anxiety is often triggered by reminders of the traumatic event and can result in pervasive feelings of worry and apprehension <a aria-describedby=\"ref-marker\">2</a>. The most prevalent among these anxiety disorders include phobias such as simple, social, and agoraphobia, as well as generalized anxiety disorder <a aria-describedby=\"ref-marker\">2</a> <a aria-describedby=\"ref-marker\">4</a>. Research also indicates that individuals with PTSD may be at a higher risk of developing OCD. Experiences of trauma and violation may lead to heightened sensitivity to perceived mental contamination and a feeling of 'dirtiness’ that may contribute to the development or exacerbation of OCD symptoms <a aria-describedby=\"ref-marker\">16</a>.</p>"

    },
    {
        name: 'Self-harm',
        kind: 'PSYCHIATRIC',
        comorbidity_percentage_lower: 5, comorbidity_percentage_higher: 19,
        risk_multiplier_lower: null,
        risk_multiplier_higher: null,
        explanation: "<p class=\"bn-inline-content\"></p><h3 class=\"bn-inline-content\">Self-harm</h3><p class=\"bn-inline-content\">Symptoms of PTSD have been identified as predictors of deliberate self-harm (DSH) <a aria-describedby=\"ref-marker\">5</a>. Depression in patients with PTSD often co-exists with suicidal ideation <a aria-describedby=\"ref-marker\">2</a>. Patients with substance use disorders (SUD) are of special interest, often showing elevated rates of suicide attempts and DSH <a aria-describedby=\"ref-marker\">5</a>.</p>"

    },
    {
        name: 'Chronic pain',
        kind: 'MEDICAL',
        comorbidity_percentage_lower: 20,
        comorbidity_percentage_higher: 20,
        risk_multiplier_lower: null,
        risk_multiplier_higher: null,
        explanation: "<p class=\"bn-inline-content\"></p><h3 class=\"bn-inline-content\">Chronic pain</h3><p class=\"bn-inline-content\">PTSD and chronic pain co-occur through various mechanisms. In PTSD, catastrophizing, characterized by magnifying pain or trauma severity, feeling overwhelmed, and expecting the worst outcome, amplifies trauma perception and emotional distress. Individuals experiencing this may exhibit less control over their pain and a greater emotional impact <a aria-describedby=\"ref-marker\">20</a>. Additionally, dysregulated cortisol levels may increase sensitivity to pain <a aria-describedby=\"ref-marker\">21</a>. Individuals experiencing both chronic pain and posttraumatic stress disorder (PTSD) tend to suffer from more intense pain and reduced quality of life compared to those solely dealing with chronic pain. Both conditions are closely linked, with high occurrences of chronic pain in PTSD patients and vice versa. Moreover, individuals with this dual diagnosis are commonly prescribed opioid medications for pain relief, putting them at a higher risk for opioid use <a aria-describedby=\"ref-marker\">22</a>.</p>"

    },
    {
        name: 'Inflammation',
        kind: 'MEDICAL',
        comorbidity_percentage_lower: 51,
        comorbidity_percentage_higher: 51,
        risk_multiplier_lower: 2,
        risk_multiplier_higher: 2,
        explanation: "<p class=\"bn-inline-content\"></p><h3 class=\"bn-inline-content\">Inflammation</h3><p class=\"bn-inline-content\">PTSD is characterized by notably heightened levels of proinflammatory markers, including interleukin-1ß, interleukin-6, tumor necrosis factor-a, and C-reactive protein <a aria-describedby=\"ref-marker\">23</a>. This inflammatory dysregulation, coupled with cognitive impairment, may collectively contribute to the symptoms observed in PTSD <a aria-describedby=\"ref-marker\">2</a>.</p>"

    },
    {
        name: 'Cardiometabolic disorders',
        kind: 'MEDICAL',
        comorbidity_percentage_lower: 27,
        comorbidity_percentage_higher: 53,
        risk_multiplier_lower: null,
        risk_multiplier_higher: null,
        explanation: "<p class=\"bn-inline-content\"></p><h3 class=\"bn-inline-content\">Cardiometabolic disorders</h3><p class=\"bn-inline-content\">Studies have shown that PTSD is linked to higher rates of physical health issues related to immune dysregulation, including metabolic syndromes, diabetes, atherosclerotic cardiovascular disease, and autoimmune diseases <a aria-describedby=\"ref-marker\">23</a>. Additionally, individuals with PTSD have an increased risk of hypertension, hyperlipidemia, obesity, and cardiovascular disease <a aria-describedby=\"ref-marker\">24</a>. Specifically, research on female survivors of the 9/11 attacks with PTSD indicates a heightened risk of hospitalization due to heart disease <a aria-describedby=\"ref-marker\">25</a>.</p>"

    },
    {
        name: 'Dementia',
        kind: 'MEDICAL',
        comorbidity_percentage_lower: 61,
        comorbidity_percentage_higher: 61,
        risk_multiplier_lower: 1.55,
        risk_multiplier_higher: 1.55,
        explanation: "<p class=\"bn-inline-content\"></p><h3 class=\"bn-inline-content\">Dementia</h3><p class=\"bn-inline-content\">Individuals with PTSD face a 1.55x higher risk of developing dementia <a aria-describedby=\"ref-marker\">9</a>. The exact mechanisms linking PTSD to cognitive decline and dementia remain unclear <a aria-describedby=\"ref-marker\">26</a>. However, research suggests that intrusive thoughts, a key symptom of PTSD, may predict cognitive impairment years later <a aria-describedby=\"ref-marker\">26</a>.</p>"

    },
    {
        name: 'Sleep dysfunction',
        kind: 'MEDICAL',
        comorbidity_percentage_lower: 70,
        comorbidity_percentage_higher: 87,
        risk_multiplier_lower: null,
        risk_multiplier_higher: null,
        explanation: "<p class=\"bn-inline-content\"></p>\r\n<h3 class=\"bn-inline-content\">Sleep dysfunction</h3>\r\n<p class=\"bn-inline-content\">70-87% of patients diagnosed with PTSD experience sleep disturbances, including insomnia,\r\n    posttraumatic nightmares, awakenings, periodic limb movement disorder (PLMD), and obstructive sleep apnea (OSA) <a aria-describedby=\"ref-marker\">10</a>\r\n    <a aria-describedby=\"ref-marker\">17</a> <a aria-describedby=\"ref-marker\">18</a>. Posttraumatic nightmares often involve reliving the trauma and can lead to intense fear or anxiety upon\r\n    waking, making it difficult to return to sleep <a aria-describedby=\"ref-marker\">19</a>. Awakenings in PTSD may not always be linked to distressing\r\n    dreams. About 33% of PTSD patients also suffer from PLMD, which can cause awakenings <a aria-describedby=\"ref-marker\">18</a>. Insomnia symptoms are\r\n    reported by approximately 70% of patients and are often related to increased autonomic arousal and fear of sleep,\r\n    including fear of losing control or experiencing nightmares <a aria-describedby=\"ref-marker\">18</a>. OSA is also common among PTSD patients, affecting\r\n    40-90% of patients <a aria-describedby=\"ref-marker\">18</a>.</p>"

    },
]






export { comorbidities };


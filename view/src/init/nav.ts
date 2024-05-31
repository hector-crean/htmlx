import { goTo } from "@/router";




const page = {
    init: () => {
        document?.querySelector("#ptsd_clinical_presentation_symptoms_page").addEventListener("click", () => {
            goTo("/ptsd/clinical_presentation/symptoms/page");
        });

        document?.querySelector("#ptsd_clinical_presentation_comorbidities_page").addEventListener("click", () => {
            goTo("/ptsd/clinical_presentation/comorbidities/page");
        });







    },
};



export default page
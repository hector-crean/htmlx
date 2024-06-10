import { router } from "@/main";




const page = {
    init: () => {
        document?.querySelector("#symptoms")?.addEventListener("click", () => {
            router.redirectTo("/ptsd/clinical_presentation/symptoms");
        });

        document?.querySelector("#comorbidities")?.addEventListener("click", () => {
            router.redirectTo("/ptsd/clinical_presentation/comorbidities");
        });

        document?.querySelector("#stigma")?.addEventListener("click", () => {
            router.redirectTo("/ptsd/diagnosis/stigma");
        });







    },
};



export default page
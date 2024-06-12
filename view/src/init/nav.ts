import { router } from "@/main";

import { routes } from '@/main';



const page = {
    init: () => {

        routes.forEach(route => {
            document?.querySelector(`#${route.id}`)?.addEventListener("click", () => {
                router.redirectTo(route.path);
            });
        })








    },
};



export default page
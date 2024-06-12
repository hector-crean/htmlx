import tabs from "@/components/tabs";
import Swiper from "swiper";


const page = {
    init: () => {

        const swiper = new Swiper('.swiper', {
            // configure Swiper to use modules
            pagination: {
              el: ".swiper-pagination",
            },
            navigation: {
              nextEl: ".swiper-button-next",
              prevEl: ".swiper-button-prev",
            },
          });

          tabs.init()





    },
};



export default page
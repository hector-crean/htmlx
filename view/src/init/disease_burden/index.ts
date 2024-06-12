import tabs from '@/components/tabs';
import Swiper from 'swiper/bundle';






export default {
    init: () => {
        // init Swiper:
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


    }
}

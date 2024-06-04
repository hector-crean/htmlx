import './style.css';
import './style/components.css';

// core version + navigation, pagination modules:
import '@/components/modal';
import '@/components/svg-container';
import 'swiper/css/bundle';

import Router from "yourrouter";



import comborbs_init from '@/init/clinical_presentation/comorbidities';
import symptoms_init from '@/init/clinical_presentation/symptoms';
import nav_init from '@/init/nav';
import { RouterConfig } from 'yourrouter/types';


export const fetchTemplate = async (location: string): Promise<string> => {
    const response = await fetch(location);
    return response.text();
  };




  enum HTML_IDS {
    APP = 'app',
  }

 const renderInHtml = async (template: () => Promise<string>, renderId: HTML_IDS): Promise<void> => {
  const container: HTMLElement = document.getElementById(renderId)!
  container.innerHTML = await template()
 }

 type Route = {
    path: `/${string}`,
    title: string;
    template: () => Promise<string>;
    scripts?: (() => void)[];
    condition?: () => boolean;
    fallback?: string;
    
  };
  

const routes: Array<Route> = [
    {
        path: '/',
        title: 'Home',
        template: () => fetchTemplate('/nav.html'),
        scripts: [nav_init.init],
        fallback: '/'
    },
    {
        path: "/ptsd/clinical_presentation/symptoms/page",
        title: 'Symptoms',
        template: () => fetchTemplate('/ptsd/clinical_presentation/symptoms/page.html'),
        scripts: [symptoms_init.init],
    },
    {
        path:  "/ptsd/clinical_presentation/comorbidities/page",
        title: 'Symptoms',
        template: () => fetchTemplate("/ptsd/clinical_presentation/comorbidities/page.html"),
        scripts: [comborbs_init.init],

    }
]


const routerConfig: RouterConfig = { path404: '/',  renderId: HTML_IDS.APP}
const router = Router.create(routerConfig);


for (const route of routes) {
   
    router.addRoute(route.path, async () => {
        await renderInHtml(route.template, HTML_IDS.APP)
        route.scripts?.forEach(script => script())
        // return new Promise()
      })
}


export { router };
 


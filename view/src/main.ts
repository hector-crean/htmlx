import './style.css';
import './style/components.css';


// core version + navigation, pagination modules:
// import '@/components/modal';
import '@/webcomponents';

import Router from "yourrouter";



import comborbs_init from '@/init/clinical_presentation/comorbidities';
import symptoms_init from '@/init/clinical_presentation/symptoms';
import nav_init from '@/init/nav';
import { RouterConfig } from 'yourrouter/types';

import nav from '@/assets/pages/nav.html?raw';
import comorbidities_page from '@/assets/pages/ptsd/clinical_presentation/comorbidities/page.html?raw';
import symptoms_page from '@/assets/pages/ptsd/clinical_presentation/symptoms/page.html?raw';
import sandbox_page from '@/assets/pages/sandbox.html?raw';

export const fetchTemplate = async (location: string): Promise<string> => {
    const response = await fetch(location);
    return response.text();
  };




  enum HTML_IDS {
    APP = 'app',
  }

 const renderHtml =  (template: string, renderId: HTML_IDS): void => {
  const container: HTMLElement = document.getElementById(renderId)!
  container.innerHTML = template
 }

 type Route = {
    path: `/${string}`,
    title: string;
    template: string;
    scripts?: (() => void)[];
    condition?: () => boolean;
    fallback?: string;
    
  };
  

const routes: Array<Route> = [
    {
        path: '/',
        title: 'Home',
        template: nav,
        scripts: [nav_init.init],
    },
    {
        path: "/ptsd/clinical_presentation/symptoms/page",
        title: 'Symptoms',
        template: symptoms_page,
        scripts: [symptoms_init.init],
    },
    {
        path:  "/ptsd/clinical_presentation/comorbidities/page",
        title: 'Symptoms',
        template: comorbidities_page,
        scripts: [comborbs_init.init],

    },
    {
      path:  "/sandbox",
      title: 'Sandbox',
      template: sandbox_page,
      scripts: [],

  }
]


const routerConfig: RouterConfig = { path404: '/',  renderId: HTML_IDS.APP}
const router = Router.create(routerConfig);





for (const route of routes) {

    router.addRoute(route.path, () => {
        renderHtml(route.template, HTML_IDS.APP)
        route.scripts?.forEach(script => script())
      })
}


export { router };
 


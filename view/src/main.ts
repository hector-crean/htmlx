import './style.css';
import './style/components.css';


// core version + navigation, pagination modules:
// import '@/components/modal';
import '@/webcomponents';

import Router from "yourrouter";
import { RouterConfig } from 'yourrouter/types';


//Init
import comborbs_init from '@/init/clinical_presentation/comorbidities';
import symptoms_init from '@/init/clinical_presentation/symptoms';
import stigma_init from '@/init/diagnosis/stigma';
import disease_burden_init from '@/init/disease_burden';
import nav_init from '@/init/nav';
import specific_populations_init from '@/init/specific_populations';
//Templates
import nav from '@/assets/pages/nav.html?raw';
import clinical_presentation_comorbidities_page from '@/assets/pages/ptsd/clinical_presentation/comorbidities/page.html?raw';
import clinical_presentation_symptoms_page from '@/assets/pages/ptsd/clinical_presentation/symptoms/page.html?raw';
import assessment_and_diagnosis_page from '@/assets/pages/ptsd/diagnosis/assessment_and_diagnosis/page.html?raw';
import diagnosis_stigma_page from '@/assets/pages/ptsd/diagnosis/stigma/page.html?raw';

import disease_burden from '@/assets/pages/ptsd/disease_burden/page.html?raw';
import specific_populations from '@/assets/pages/ptsd/specific_populations/page.html?raw';


export const fetchTemplate = async (location: string): Promise<string> => {
  const response = await fetch(location);
  return response.text();
};




enum HTML_IDS {
  APP = 'app',
}

const renderHtml = (template: string, renderId: HTML_IDS): void => {
  const container: HTMLElement = document.getElementById(renderId)!
  container.innerHTML = template
}

type Route = {
  path: `/${string}`,
  id: string;
  template: string;
  scripts?: (() => void)[];
  condition?: () => boolean;
  fallback?: string;

};


const routes: Array<Route> = [
  {
    path: '/',
    id: 'home',
    template: nav,
    scripts: [nav_init.init],
  },
  {
    path: "/ptsd/clinical_presentation/symptoms",
    id: 'symptoms',
    template: clinical_presentation_symptoms_page,
    scripts: [symptoms_init.init],
  },
  {
    path: "/ptsd/clinical_presentation/comorbidities",
    id: 'comorbidities',
    template: clinical_presentation_comorbidities_page,
    scripts: [comborbs_init.init],

  },
  {
    path: "/ptsd/diagnosis/stigma",
    id: 'stigma',
    template: diagnosis_stigma_page,
    scripts: [stigma_init.init],
  },
  {
    path: '/ptsd/diagnosis/assessment_and_diagnosis',
    id: 'assessment_and_diagnosis',
    template: assessment_and_diagnosis_page,
    scripts: []
  },
  {
    path: '/ptsd/specific_populations',
    id: 'specific_populations',
    template: specific_populations,
    scripts: [specific_populations_init.init]
  },
  {
    path: '/ptsd/disease_burden',
    id: 'disease_burden',
    template: disease_burden,
    scripts: [disease_burden_init.init]
  },
  
 
]


const routerConfig: RouterConfig = { path404: '/', renderId: HTML_IDS.APP }
const router = Router.create(routerConfig);





for (const route of routes) {

  router.addRoute(route.path, () => {
    renderHtml(route.template, HTML_IDS.APP)
    route.scripts?.forEach(script => script())
  })
}


export { router, routes };



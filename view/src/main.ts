import './style.css';
import './style/components.css';

import "@/lib/webcomponents";

// core version + navigation, pagination modules:
// import '@/components/modal';

import Router from "yourrouter";
import { RouterConfig } from 'yourrouter/types';


//Init
import nav_init from '@/nav';
import clinical_course_init from '@/pages/ptsd/clinical_course/page';
import comborbs_init from '@/pages/ptsd/clinical_presentation/comorbidities/page';
import symptoms_init from '@/pages/ptsd/clinical_presentation/symptoms/page';
import stigma_init from '@/pages/ptsd/diagnosis/stigma/page';
import disease_burden_init from '@/pages/ptsd/disease_burden/page';
import specific_populations_init from '@/pages/ptsd/specific_populations/page';
//Templates
import nav from '@/pages/nav.html?raw';
import clinical_presentation_comorbidities_page from '@/pages/ptsd/clinical_presentation/comorbidities/page.html?raw';
import clinical_presentation_symptoms_page from '@/pages/ptsd/clinical_presentation/symptoms/page.html?raw';
import assessment_and_diagnosis_page from '@/pages/ptsd/diagnosis/assessment_and_diagnosis/page.html?raw';
import diagnosis_stigma_page from '@/pages/ptsd/diagnosis/stigma/page.html?raw';

import clinical_course_page from '@/pages/ptsd/clinical_course/page.html?raw';

import disease_burden from '@/pages/ptsd/disease_burden/page.html?raw';
import specific_populations from '@/pages/ptsd/specific_populations/page.html?raw';

import pathophysiology_of_ptsd_init from '@/pages/ptsd/disease/pathophysiology_of_ptsd/page';
import pathophysiology_of_ptsd from '@/pages/ptsd/disease/pathophysiology_of_ptsd/page.html?raw';

import trauma_types_init from '@/pages/ptsd/disease/trauma_types/page';
import trauma_types from '@/pages/ptsd/disease/trauma_types/page.html?raw';


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
  {
    path: '/ptsd/disease/pathophysiology_of_ptsd',
    id: 'pathophysiology_of_ptsd',
    template: pathophysiology_of_ptsd,
    scripts: [pathophysiology_of_ptsd_init.init]
  },
  {
    path: '/ptsd/disease/trauma_types',
    id: 'trauma_types',
    template: trauma_types,
    scripts: [trauma_types_init.init]
  },
  {
    path: '/ptsd/clinical_course',
    id: 'clinical_course',
    template: clinical_course_page,
    scripts: [clinical_course_init.init]
  }
  
 
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



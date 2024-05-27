import initClinicalPresentationComorbidities from './init/clinical_presentation/comorbidities';
import initClinicalPresentationSymptoms from './init/clinical_presentation/symptoms';
// src/router.ts
type Route = {
    path: string;
    template: string;
    init: () => void;
  };
  
  
  const routes: Route[] = [
    {
      path: "/",
      template: "/nav.html",
      init: () => {}
    },
    {
      path: "/ptsd/disease/trauma_types/page",
      template: "/ptsd/disease/trauma_types/page.html", 
      init: () => {},
    },
    {
      path: "/ptsd/disease/pathophysiology_of_ptsd/page",
      template: "/ptsd/disease/pathophysiology_of_ptsd/page.html", 
      init: () => {},
    },
    {
      path: "/ptsd/specific_populations/civilian_vs_military/page",
      template: "/ptsd/specific_populations/civilian_vs_military/page.html", 
      init: () => {},
    },
    {
      path: "/ptsd/specific_populations/marginalized_groups/page",
      template: "/ptsd/specific_populations/marginalized_groups/page.html", 
      init: () => {},
    },
    {
      path: "/ptsd/clinical_presentation/symptoms/page",
      template: "/ptsd/clinical_presentation/symptoms/page.html", 
      init: () => {
        initClinicalPresentationSymptoms.init()
      },
    },
    {
      path: "/ptsd/clinical_presentation/comorbidities/page",
      template: "/ptsd/clinical_presentation/comorbidities/page.html", 
      init: () => {
        initClinicalPresentationComorbidities.init()
      },
    },
    {
      path: "/ptsd/disease_burden/personal_burden/page",
      template: "/ptsd/disease_burden/personal_burden/page.html", 
      init: () => {},
    },
    {
      path: "/ptsd/disease_burden/societal_burden/page",
      template: "/ptsd/disease_burden/societal_burden/page.html", 
      init: () => {},
    },
    {
      path: "/ptsd/diagnosis/assessment_and_diagnosis/page",
      template: "/ptsd/diagnosis/assessment_and_diagnosis/page.html", 
      init: () => {},
    },
    {
      path: "/ptsd/diagnosis/stigma/page",
      template: "/ptsd/diagnosis/stigma/page.html", 
      init: () => {},
    },
    {
      path: "/ptsd/diagnosis/interviews_with_clinicians/page",
      template: "/ptsd/diagnosis/interviews_with_clinicians/page.html", 
      init: () => {},
    },
    {
      path: "/ptsd/clinical_course/delayed_onset_ptsd/page",
      template: "/ptsd/clinical_course/delayed_onset_ptsd/page.html", 
      init: () => {},
    },
    {
      path: "/ptsd/clinical_course/chronic_ptsd/page",
      template: "/ptsd/clinical_course/chronic_ptsd/page.html", 
      init: () => {},
    },
    {
      path: "/ptsd/clinical_course/underdiagnosis/page",
      template: "/ptsd/clinical_course/underdiagnosis/page.html", 
      init: () => {},
    },
    {
      path: "/ptsd/treatment/guidelines/page",
      template: "/ptsd/treatment/guidelines/page.html", 
      init: () => {},
    },
    {
      path: "/ptsd/treatment/unmet_needs_and_barriers/page",
      template: "/ptsd/treatment/unmet_needs_and_barriers/page.html", 
      init: () => {},
    },
    {
      path: "/ptsd/treatment/trauma_informed_care/page",
      template: "/ptsd/treatment/trauma_informed_care/page.html", 
      init: () => {},
    },
    {
      path: "/ptsd/recovery/intermediate_recovery/page",
      template: "/ptsd/recovery/intermediate_recovery/page.html", 
      init: () => {},
    },
    {
      path: "/ptsd/recovery/long_term_reconstruction/page",
      template: "/ptsd/recovery/long_term_reconstruction/page.html", 
      init: () => {},
    },
    {
      path: "/ptsd/home",
      template: "/ptsd/home.html", 
      init: () => {},
    }
  ]
  
  const loadRoute = async (path: string) => {
    const route = routes.find(route => route.path === path);
    if (route) {
      const response = await fetch(route.template);
      const html = await response.text();
      document.getElementById('app')!.innerHTML = html;
    } else {
      document.getElementById('app')!.innerHTML = '<h1>Page Not Found</h1>';
    }
  };
  
  export const handleNavigation = (event: Event) => {
    event.preventDefault();
    const target = event.target as HTMLAnchorElement;
    const path = target.getAttribute('href');
    history.pushState({}, '', path);
    loadRoute(path).then(() => {
      const route = routes.find(route => route.path === path);
      console.log(route)

      route?.init()
    })
   


  };
  
  export const initializeRouter = () => {
    window.addEventListener('popstate', () => loadRoute(location.pathname));
    document.addEventListener('DOMContentLoaded', () => {
      document.querySelectorAll('a').forEach(anchor => {
        anchor.addEventListener('click', handleNavigation);
      });

      loadRoute(location.pathname).then(() => {
        const route = routes.find(route => route.path === location.pathname);
        console.log(route)
  
        route?.init()
      })



    });
  };
  
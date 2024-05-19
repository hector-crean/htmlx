// src/router.ts
type Route = {
    path: string;
    template: string;
  };
  
  const routes: Route[] = [
    {
      "path": "/PTSD/disease/trauma_types/page",
      "template": "/PTSD/disease/trauma_types/page.html"
    },
    {
      "path": "/PTSD/disease/pathophysiology_of_ptsd/page",
      "template": "/PTSD/disease/pathophysiology_of_ptsd/page.html"
    },
    {
      "path": "/PTSD/specific_populations/civilian_vs_military/page",
      "template": "/PTSD/specific_populations/civilian_vs_military/page.html"
    },
    {
      "path": "/PTSD/specific_populations/marginalized_groups/page",
      "template": "/PTSD/specific_populations/marginalized_groups/page.html"
    },
    {
      "path": "/PTSD/clinical_presentation/symptoms/page",
      "template": "/PTSD/clinical_presentation/symptoms/page.html"
    },
    {
      "path": "/PTSD/clinical_presentation/comorbidities/page",
      "template": "/PTSD/clinical_presentation/comorbidities/page.html"
    },
    {
      "path": "/PTSD/disease_burden/personal_burden/page",
      "template": "/PTSD/disease_burden/personal_burden/page.html"
    },
    {
      "path": "/PTSD/disease_burden/societal_burden/page",
      "template": "/PTSD/disease_burden/societal_burden/page.html"
    },
    {
      "path": "/PTSD/diagnosis/assessment_and_diagnosis/page",
      "template": "/PTSD/diagnosis/assessment_and_diagnosis/page.html"
    },
    {
      "path": "/PTSD/diagnosis/stigma/page",
      "template": "/PTSD/diagnosis/stigma/page.html"
    },
    {
      "path": "/PTSD/diagnosis/interviews_with_clinicians/page",
      "template": "/PTSD/diagnosis/interviews_with_clinicians/page.html"
    },
    {
      "path": "/PTSD/clinical_course/delayed_onset_ptsd/page",
      "template": "/PTSD/clinical_course/delayed_onset_ptsd/page.html"
    },
    {
      "path": "/PTSD/clinical_course/chronic_ptsd/page",
      "template": "/PTSD/clinical_course/chronic_ptsd/page.html"
    },
    {
      "path": "/PTSD/clinical_course/underdiagnosis/page",
      "template": "/PTSD/clinical_course/underdiagnosis/page.html"
    },
    {
      "path": "/PTSD/treatment/guidelines/page",
      "template": "/PTSD/treatment/guidelines/page.html"
    },
    {
      "path": "/PTSD/treatment/unmet_needs_and_barriers/page",
      "template": "/PTSD/treatment/unmet_needs_and_barriers/page.html"
    },
    {
      "path": "/PTSD/treatment/trauma_informed_care/page",
      "template": "/PTSD/treatment/trauma_informed_care/page.html"
    },
    {
      "path": "/PTSD/recovery/intermediate_recovery/page",
      "template": "/PTSD/recovery/intermediate_recovery/page.html"
    },
    {
      "path": "/PTSD/recovery/long_term_reconstruction/page",
      "template": "/PTSD/recovery/long_term_reconstruction/page.html"
    },
    {
      "path": "/PTSD/home",
      "template": "/PTSD/home.html"
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
    loadRoute(path);
  };
  
  export const initializeRouter = () => {
    window.addEventListener('popstate', () => loadRoute(location.pathname));
    document.addEventListener('DOMContentLoaded', () => {
      document.querySelectorAll('a').forEach(anchor => {
        anchor.addEventListener('click', handleNavigation);
      });
      loadRoute(location.pathname);
    });
  };
  
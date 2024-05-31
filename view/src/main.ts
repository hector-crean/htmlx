import './style.css';
import './style/components.css';


import comborbs_init from '@/init/clinical_presentation/comorbidities';
import symptoms_init from '@/init/clinical_presentation/symptoms';
import nav_init from '@/init/nav';
import router, { getFile, Routes, setRoutes } from './router';

const appEl = document.querySelector<HTMLDivElement>('#app')!;


const routes: Routes = {
    "/": {
        title: 'Home',
        content: getFile('/nav.html'),
        scripts: [nav_init.init],
        fallback: '/'
    },
    "/ptsd/clinical_presentation/symptoms/page": {
        title: 'Symptoms',
        content: getFile('/ptsd/clinical_presentation/symptoms/page.html'),
        scripts: [symptoms_init.init],
    },
    "/ptsd/clinical_presentation/comorbidities/page": {
        title: 'Symptoms',
        content: getFile("/ptsd/clinical_presentation/comorbidities/page.html"),
        scripts: [comborbs_init.init],

    }

};

setRoutes(routes);

router();


export { appEl };

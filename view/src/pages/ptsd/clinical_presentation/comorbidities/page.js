
import { BarChart, barChartOrd } from '@/lib/components/bar-chart-interval';
import { comorbidities } from './comorbidities';

const page = {
    init: () => {

        const stackedContainer = document.querySelector('#ptsd-comorbidities-bar-chart')
        new BarChart(comorbidities, stackedContainer, barChartOrd);



    },
};



export default page
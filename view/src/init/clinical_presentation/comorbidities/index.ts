
import { BarChart, barChartOrd } from '@/components/charts/bar-chart-interval';
import { comorbidities } from '@/data/comborbidities';

const page = {
    init: () => {

        const stackedContainer = document.querySelector<HTMLElement>('#ptsd-comorbidities-bar-chart')!

        new BarChart(comorbidities, stackedContainer, barChartOrd);

      

    },
};



export default page
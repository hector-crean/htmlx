
import { StackedBarChart, barChartOrd, medicalCormidities } from '@/components/charts/stacked-bar-chart';
import { penguinDotPlot, penguinStackPlot } from "@/components/plot/stack";

const page = {
    init: () => {
        const sandboxContainer = document.querySelector<HTMLElement>('#sandbox-container')!;
       
        const stackedContainer = sandboxContainer.querySelector<HTMLElement>('#stacked-bar-chart-1')!
        
        const stackedChart = new StackedBarChart(medicalCormidities, stackedContainer, barChartOrd);

        sandboxContainer.append(penguinStackPlot)
        sandboxContainer.append(penguinDotPlot)
        // const stackedContainer = sandboxContainer.querySelector<HTMLElement>('#stacked-bar-chart-1')!;
        // new StackedBarChart(stackedData, stackedContainer, barChartOrd)


       
          

    },
};



export default page
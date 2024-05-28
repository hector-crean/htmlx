import { StackedBarChart, barChartOrd, stackedData } from "@/components/charts/stacked-bar-chart";



const page = {
    init: () => {
        const sandboxContainer = document.querySelector<HTMLElement>('#sandbox-container')!;
       
        const stackedContainer = sandboxContainer.querySelector<HTMLElement>('#stacked-bar-chart-1')!;

        new StackedBarChart(stackedData, stackedContainer, barChartOrd)
    },
};



export default page
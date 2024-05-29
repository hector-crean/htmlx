
import { penguinDotPlot, penguinRadarPlot, penguinStackPlot } from "@/components/plot/stack";

const page = {
    init: () => {
        const sandboxContainer = document.querySelector<HTMLElement>('#sandbox-container')!;
       
        sandboxContainer.append(penguinStackPlot)
        sandboxContainer.append(penguinDotPlot)
        sandboxContainer.append(penguinRadarPlot)
        // const stackedContainer = sandboxContainer.querySelector<HTMLElement>('#stacked-bar-chart-1')!;
        // new StackedBarChart(stackedData, stackedContainer, barChartOrd)


       
          

    },
};



export default page
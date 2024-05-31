
import { BarChart, barChartOrd } from '@/components/charts/bar-chart-interval';
import { comorbidities } from '@/data/comborbidities';
import * as Plot from '@observablehq/plot';

const page = {
    init: () => {
        const sandboxContainer = document.querySelector<HTMLElement>('#sandbox-container')!;

        const stackedContainer = sandboxContainer.querySelector<HTMLElement>('#stacked-bar-chart-1')!

        const barChart = new BarChart(comorbidities, stackedContainer, barChartOrd);

        const covariancePlot = Plot.plot({

            marks: [
                Plot.dot(
                    comorbidities,
                    {

                        x: 'comorbidity_percentage_lower',
                        y: 'risk_multiplier_lower',

                        stroke: "kind",
                        tip: {
                            fill: 'black'
                        },
                        channels: { name: "name", kind: "kind" },

                    }),
                Plot.crosshair(comorbidities, {
                    x: 'comorbidity_percentage_lower',
                    y: 'risk_multiplier_lower',
                })
            ]
        })

        sandboxContainer.append(covariancePlot)





    },
};



export default page
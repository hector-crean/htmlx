import barChart from "@/components/charts/horizontal-correlation-chart";
// import neuroComorbData from "./comorbs-neuro-data.json";
// import physicalComorbData from "./comorbs-physical-data.json";

import medical_comborbidities_bars from './medical_comorbidities_bars.json';
import psychiatric_comborbidities_bars from './psychiatric_comborbidities_bars.json';


const renderRichText = (block) => {
    switch (block.type) {
        case 'RichTextBlock':
            switch (block.props.text.type) {
                case 'Html': {
                    return `${block.props.text.data}`
                }
                default:
                    return ''
            }
        default:
            return ''
    }
}

export default {
    init: () => {
        const style = {
            barWidth: 320,
            textWidth: 256,
            textSize: 12,
            barHeight: 42,
            iconSize: 36,
            barPadding: 0.2,
            axisSize: 20,
        };

        const icons = (str: string) => ''

        const tooltip1 = "Tap or click on a condition for more information";


        const selectedText = (d) =>
            `<b>${d.percent}%</b> of patients with currently active PTSD and past PTSD had <b>${d.shortTitle}</b> as a comorbidity.<br><br><div class="text-left"><span>${renderRichText(d.content)}</span>`;



        barChart.GenerateGraph(
            "#psychiatric-comorbidities",
            "#info-psychiatric-comorbidities",
            psychiatric_comborbidities_bars,
            style,
            {
                tooltip: tooltip1,
                selected: selectedText,
            },
            icons,
        );

        const tooltip2 = "Tap or click on a condition for more information";


        barChart.GenerateGraph(
            "#medical-comorbidities",
            "#info-medical-comorbidities",
            medical_comborbidities_bars,
            style,
            {
                tooltip: tooltip2,
                selected: selectedText,
            },
            icons,
        );

    },
};

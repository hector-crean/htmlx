import { PieChartDatum } from "@/webcomponents/pie-chart";

const data: Array<PieChartDatum> = [
    { id: '1', label: 'Excess direct health care costs', value: 32.8 },
    { id: '2', label: 'Excess direct non-health care costs', value: 15.4 },
    { id: '3', label: 'Excess costs of unemployment', value: 19.9 },
    { id: '4', label: 'Excess costs of productivity loss', value: 15 },
    { id: '5', label: 'Excess costs due to caregiving', value: 15.8 },
    { id: '6', label: 'Excess costs of premature mortality', value: 1.1 }

];


const page = {
    init: () => {
        // const pieChart = document.querySelector('pie-chart') as any;
        // pieChart.data = data;

    },
};



export default page
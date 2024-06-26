import { max } from 'd3-array';
import { axisBottom, axisLeft } from 'd3-axis';
import { ScaleBand, ScaleLinear, scaleBand, scaleLinear } from 'd3-scale';
import { Selection, select } from 'd3-selection';
import { Observable, fromEvent } from 'rxjs';
import { debounceTime } from 'rxjs/operators';
import { Ord } from './ord';
import { ChartSize, Margin } from './types';




interface BarChartData {
    label: string;
    value: number;
}

class BarChart<T extends BarChartData> {
    private container: HTMLElement;
    private margin: Margin;
    private data: BarChartData[];
    private svg: Selection<SVGSVGElement, unknown, null, any>;
    private scaleY: ScaleBand<string>;
    private scaleX: ScaleLinear<number, number, never>
    private size: ChartSize
    private resizeObservable$: Observable<Event>;


    constructor(data: T[], container: HTMLElement, ord: Ord<T>, margin: Margin = { top: 20, right: 30, bottom: 40, left: 40 }) {
        this.margin = margin
        this.data = data.sort(ord.compare);
        this.container = container;

        this.size = this.calculateSize(container, margin);

        this.scaleY = scaleBand()
            .domain(this.data.map(d => d.label))
            .padding(0.1)
            .range([0, this.size.innerHeight]);

        this.scaleX = scaleLinear()
            .domain([0, max(this.data, d => d.value)!])
            .nice()
            .range([0, this.size.innerWidth]);

        this.svg = select(container)
            .append('svg')
            .attr('width', '100%')
            .attr('height', '100%')
            .attr('viewport', `"${0} ${0} ${this.size.innerWidth} ${this.size.innerHeight}"`);

        this.draw();

        this.resizeObservable$ = fromEvent(window, 'resize').pipe(debounceTime(200));
        this.resizeObservable$.subscribe(() => {
            this.onResize();
        });

    }

    private draw(): void {
        this.svg.selectAll('*').remove();

        const g = this.svg.append('g')
            .attr('transform', `translate(${this.margin.left},${this.margin.top})`);


        g.append('g')
            .attr('class', 'x-axis')
            .attr('transform', `translate(0,${this.size.innerHeight})`)
            .call(axisBottom(this.scaleX));

        g.append('g')
            .attr('class', 'y-axis')
            .call(axisLeft(this.scaleY).ticks(10, 's'));

        g.selectAll('.bar')

            .data(this.data)
            .enter()
            .append('rect')
            .attr('class', 'bar')
            .attr('x', d => 0)
            .attr('y', d => this.scaleY(d.label)!)
            .attr('height', this.scaleY.bandwidth()) 
            .transition()
            .duration(800)
            .attr('width', d => this.scaleX(d.value))
           
    }

    private onResize(): void {
        this.size = this.calculateSize(this.container, this.margin);

        this.scaleX.range([0, this.size.innerWidth]);
        this.scaleY.range([this.size.innerHeight, 0]);

        this.svg
            .attr('width', '100%')
            .attr('height', '100%')
            .attr('viewport', `"${0} ${0} ${this.size.innerWidth} ${this.size.innerHeight}"`);

        this.draw();
    }



    private calculateSize(containerEl: HTMLElement, margin: Margin,): ChartSize {
        const rect = containerEl.getBoundingClientRect();
        const innerWidth = rect.width - margin.left - margin.right;
        const innerHeight = rect.height - margin.top - margin.bottom;

        return {
            innerWidth, innerHeight, outerWidth: rect.width, outerHeight: rect.height
        }

    }


}

const data: BarChartData[] = [
    { label: 'A', value: 30 },
    { label: 'B', value: 80 },
    { label: 'C', value: 45 },
    { label: 'D', value: 60 },
    { label: 'E', value: 20 },
    { label: 'F', value: 90 },
    { label: 'G', value: 55 },
];
const barChartOrd: Ord<BarChartData> = {
    equals: (x, y) => x === y,
    compare: (first, second) => first.value > second.value ? 1 : first.value < second.value ? -1 : 0
}

export { BarChart, barChartOrd, data };


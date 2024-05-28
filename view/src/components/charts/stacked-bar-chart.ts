import { axisBottom, axisLeft } from 'd3-axis';
import { } from 'd3-color';
import { ScaleBand, ScaleLinear, ScaleOrdinal, scaleBand, scaleLinear, scaleOrdinal } from 'd3-scale';
import { Selection, select, selectAll } from 'd3-selection';
import { Series, stack } from 'd3-shape';
import { Observable, fromEvent } from 'rxjs';
import { debounceTime } from 'rxjs/operators';
import { Ord } from './ord';
import pattern, { PatternFunction } from './textures';
import { ChartSize, Margin } from './types';

const blues = [
               
    "#7ea8cb",
    "#8EC1D3",
    "#9FD6DB",
    "#AFE2DD",
    "#C0E9DD",
    "#D2F0E2",
    "#E4F6EA"
    ]


const colors = [
    "#FF5733", // Bright Red
    "#FF8D33", // Orange
    "#FFC733", // Yellow
    "#E5FF33", // Lime
    "#8DFF33", // Light Green
    "#33FF57", // Green
    "#33FF8D", // Mint
    "#33FFC7", // Aqua
    "#33E5FF", // Light Blue
    "#338DFF", // Blue
    "#5733FF", // Indigo
    "#8D33FF", // Purple
    "#C733FF", // Violet
    "#FF33E5", // Pink
    "#FF338D"  // Magenta
  ];

  

/**
 * https://riccardoscalco.it/textures/
 */

type StackedBarChartData = {
    group: string;
    values: {[key: string]: number};
};

class StackedBarChart<T extends StackedBarChartData> {
    private container: HTMLElement;
    private margin: Margin;
    private data: StackedBarChartData[];
    private stackedData: Series<{
        [key: string]: number;
    }, string>[]
    private svg: Selection<SVGSVGElement, unknown, null, any>;
    private scaleY: ScaleBand<string>;
    private scaleX: ScaleLinear<number, number, never>
    private colorScale: ScaleOrdinal<string, unknown, never>
    private size: ChartSize
    private resizeObservable$: Observable<Event>;
    private mask: Selection<SVGMaskElement, unknown, null, any>
    private pattern: PatternFunction


    constructor(data: T[], container: HTMLElement, ord: Ord<T>, margin: Margin = { top: 20, right: 30, bottom: 40, left: 200 }) {
        
      
        const sortedData = data.sort(ord.compare);
        this.data = sortedData;

        this.margin = margin
        
        this.container = container;

        this.size = this.calculateSize(container, margin);

        const groups = data.map(d => d.group)
        const subgroups = Object.keys(data[0].values)

        this.scaleY = scaleBand()
            .domain(groups)
            .padding(0.1)
            .range([0, this.size.innerHeight]);

        const xs = data.flatMap(datum => Object.values(datum.values).reduce((sum, x) => sum + x, 0))

        this.scaleX = scaleLinear()
            .domain([0, Math.max(...xs)])
            .nice()
            .range([0, this.size.innerWidth]);

        
       
        const colorScale = scaleOrdinal()
            .domain(subgroups)
            .range(blues);

        this.colorScale = colorScale
        

        const stackedData = stack().keys(subgroups)(sortedData.map(data => ({...data.values, group: data.group})))
        this.stackedData = stackedData

        const pattern1 = pattern.lines()
        .size(8)
        .strokeWidth(2);



        

        const svg = select(container)
            .append('svg')

        svg.call(pattern1)
           

        svg.attr('width', '100%')
            .attr('height', '100%')
            .attr('viewport', `"${0} ${0} ${this.size.innerWidth} ${this.size.innerHeight}"`);


        const mask = svg.append("defs")
        .append("mask")
        .attr("id", "myMask");

        console.log('mask', mask)

        mask.append("rect")
        .attr("x", 0)
        .attr("y", 0)
        .attr("width", 800)
        .attr("height", 500)
        .style("fill", "white")
        .style("opacity", 0.7);
        
      mask.append("circle")
        .attr("cx", 400)
        .attr("cy", 400)
        .attr("r", 250);



        this.svg = svg
        this.pattern = pattern1
        this.mask = mask;

            

        this.draw();

        this.resizeObservable$ = fromEvent(window, 'resize').pipe(debounceTime(200));
        this.resizeObservable$.subscribe(() => {
            this.onResize();
        });

    }

    private draw(): void {
        this.svg.selectAll('.viz')
        .remove();

        const g = this.svg.append('g').attr('transform', `translate(${this.margin.left},${this.margin.top})`)
        g.classed('viz');

        g.append('g')
            .attr('class', 'x-axis')
            .attr('transform', `translate(0,${this.size.innerHeight})`)
            .call(axisBottom(this.scaleX));

        g.append('g')
            .attr('class', 'y-axis')
            .call(axisLeft(this.scaleY).ticks(10, 's'));


        g.selectAll('.bar')

            .data(this.stackedData)
            .join("g")
            .attr("fill", d => {
                switch(d.key){
                    case 'baseline':
                        return this.colorScale(d.key)
                    case 'uncertainty':
                        return this.pattern.url()
                    default:
                        return this.pattern.url()
                }
            })
            .attr("class", d => `myRect ${d.key}` ) // Add a class to each subgroup: their name
            .selectAll("rect")
            // enter a second time = loop subgroup per subgroup to add all rectangles
            .data(d => d)
            .join("rect")
            .on('pointerdown', (e,d) => console.log(e))
              .attr("x", d => this.scaleX(d[0]))
              .attr("y", d => this.scaleY(d.data.group)!)
              .attr("height", d => this.scaleY.bandwidth())
              .attr("width", d => this.scaleX(d[1]) - this.scaleX(d[0]))
              .attr("stroke", "grey")
              .attr("mask", "url(#myMask)")
              .on("mouseover", (event, d) => { // What happens when user hover a bar
      
                console.log(event, d)
                // what subgroup are we hovering?
                const subGroupName = select(this.parentNode).datum().key
      
                // Reduce opacity of all rect to 0.2
                 selectAll(".myRect").style("opacity", 0.2)
      
                // Highlight all rects of this subgroup with opacity 1. It is possible to select them since they have a specific class = their name.
                 selectAll(`.${subGroupName}`).style("opacity",1)
              })
              .on("mouseleave", function (event,d) { // When user do not hover anymore
      
                // Back to normal opacity: 1
                selectAll(".myRect")
                .style("opacity",1)
            })
           
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

const stackedData: Array<StackedBarChartData> = [
    {
        group: 'Self-harm',
        values: {
            baseline: 60,
            uncertainty: 10,
        }
        
    },
    {
        group: 'Anxiety',
        values: {
            baseline: 40,
            uncertainty: 4,
        }
        
    },
    {
        group: 'Major Depressive Disorder',
        values: {
            baseline: 38,
            uncertainty: 7,
        }
      
    }
]
const barChartOrd: Ord<StackedBarChartData> = {
    equals: (x, y) => x === y,
    compare: (first, second) => first.values > second.values ? 1 : first.values < second.values ? -1 : 0
}

export { StackedBarChart, barChartOrd, stackedData };


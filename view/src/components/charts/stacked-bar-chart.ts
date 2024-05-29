import { axisBottom, axisLeft } from 'd3-axis';
import { } from 'd3-color';
import { format } from 'd3-format';
import { ScaleBand, ScaleLinear, ScaleOrdinal, scaleBand, scaleLinear, scaleOrdinal } from 'd3-scale';
import { Selection, select } from 'd3-selection';
import { Series, stack } from 'd3-shape';
import { Observable, fromEvent } from 'rxjs';
import { debounceTime } from 'rxjs/operators';
import { Ord } from './ord';
import pattern from './textures';
import { ChartSize, Margin } from './types';
function toKebabCase(str: string): string {
    return str
      .replace(/([a-z])([A-Z])/g, '$1-$2') // Add hyphen between lowercase and uppercase characters
      .replace(/[\s_]+/g, '-')              // Replace spaces and underscores with hyphens
      .toLowerCase();                       // Convert the entire string to lowercase
  }

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

type StackedBarChartData<Datum extends Record<string,number> = Record<string,number>> = {
    group: string;
    values: Datum
};

class StackedBarChart<Datum extends Record<string,number>, T extends StackedBarChartData<Datum>> {
    private container: HTMLElement;
    private margin: Margin;
    private data: StackedBarChartData<Datum>[];
    private stackedData: Series<{
        [key: string]: number;
    }, string>[]
    private svg: Selection<SVGSVGElement, unknown, null, any>;
    private scaleY: ScaleBand<string>;
    private scaleX: ScaleLinear<number, number, never>
    private colorScale: ScaleOrdinal<string, unknown, never>
    private size: ChartSize
    private resizeObservable$: Observable<Event>;


    constructor(data: T[], container: HTMLElement, ord: Ord<T>, margin: Margin = { top: 20, right: 30, bottom: 40, left: 200 }) {
        
      
        const sortedData = data.sort(ord.compare);
        this.data = sortedData;

        this.margin = margin
        
        this.container = container;

        this.size = this.calculateSize(container, margin);

        const groups = data.map(d => d.group)
        console.log(this.data[0])
        const subgroups = Object.keys(this.data[0].values)

        this.scaleY = scaleBand()
            .domain(groups)
            .padding(0.1)
            .range([0, this.size.innerHeight]);

        const xs = data.flatMap(datum => Object.values(datum.values).reduce((sum, x) => sum + x, 0))

        this.scaleX = scaleLinear()
            // .domain([0, Math.max(...xs)])
            .domain([0, 100])
            .nice()
            .range([0, this.size.innerWidth]);

        
       
        const colorScale = scaleOrdinal()
            .domain(subgroups)
            .range(blues);

        this.colorScale = colorScale
        

        const ds = this.data.map(data => ({...data.values, group: data.group}))
        const stackedData = stack().keys(subgroups)(ds)
        this.stackedData = stackedData

      
        const svg = select(container)
            .append('svg')

         
        
        svg.attr('width', '100%')
            .attr('height', '100%')
            .attr('viewport', `${0} ${0} ${this.size.innerWidth} ${this.size.innerHeight}`).attr('data-interactive', true)



        this.svg = svg

            

        this.draw();

        this.resizeObservable$ = fromEvent(window, 'resize').pipe(debounceTime(200));
        this.resizeObservable$.subscribe(() => {
            this.onResize();
        });

    }

    private draw(): void {

        this.svg.selectAll('*').remove();

        const defs = this.svg.append("defs");


        const flowFilter = defs.append('filter').attr("id","glow");
        flowFilter.append("feGaussianBlur")
            .attr("stdDeviation","3.5")
            .attr("result","coloredBlur");
        const feMerge = flowFilter.append("feMerge");
        feMerge.append("feMergeNode")
            .attr("in","coloredBlur");
        feMerge.append("feMergeNode")
            .attr("in","SourceGraphic");

        const pattern1 = pattern.lines()
        .size(8)
        .strokeWidth(2).stroke("#7ea8cb");
        
        this.svg.call(pattern1)

        const vizLayer = this.svg.append('g').attr('transform', `translate(${this.margin.left},${this.margin.top})`)
        vizLayer.classed('viz');

       
        const graphLayer = vizLayer.append("g")
        graphLayer.classed("graphLayer", true);

        const bgRect = graphLayer.append('rect')
        bgRect
        .attr('data-interactive', true)
        .attr('width', this.size.innerWidth)
        .attr('height', this.size.innerHeight)
        .attr('fill', 'transparent')
     

        // Bind data to masks and append mask elements with unique IDs
        
        // const masks = defs.selectAll('mask').data(this.stackedData.flat())
        // .join(
        //     enter => enter
        //     .append('mask')
        //         .attr('id', (d, i) => toKebabCase(`mask-${d.data.group}`))
        //     .append('g')
        //         .attr('transform', `translate(${this.margin.left},${this.margin.top})`)
        //     .append('rect')
        //         .attr("x", (d,i) => this.scaleX(d[0]))
        //         .attr("y", (d,i) => this.scaleY(d.data.group)!)
        //         .attr("height", d => this.scaleY.bandwidth())
        //         .attr("width", d => this.scaleX(d[1]) - this.scaleX(d[0]))
        //         .attr("fill", "white"),
        //     update =>  update,
        //     exit => exit.remove()
        // );




      
       
       

        const xAxisContainer = vizLayer.append('g')
        const xAxis = axisBottom(this.scaleX).tickFormat(n => format(".0%")(n.valueOf()/100))
        xAxisContainer.attr('class', 'x-axis')
            .attr('transform', `translate(0,${this.size.innerHeight})`)
            .call(xAxis);

        
        const yAxis = axisLeft(this.scaleY).ticks(10, 's')
        const yAxisContainer = vizLayer.append('g')
        yAxisContainer.attr('class', 'y-axis')
            .call(yAxis);

         


        const stacks = graphLayer
        .selectAll("g")
        .data(this.stackedData)
        .join("g")
        .attr("fill",  (d) => {
            switch(d.key){
                case 'baseline':
                    return this.colorScale(d.key)
                case 'uncertainty':
                    return pattern1.url()
                default:
                    return pattern1.url()
            }
        })
        
        const bars =  stacks.selectAll("rect")
        .data(D => D)
        .join('rect')
        .attr('data-interactive', true)
        .attr("x", d => this.scaleX(d[0]))
        .attr("y", d => this.scaleY(d.data.group)!)
        .attr("height", d => this.scaleY.bandwidth())
        .attr("mask", d => `url(#${toKebabCase(`mask-${d.data.group}`)})`)
        .attr('width', d => this.scaleX(d[1]) - this.scaleX(d[0]))

      
               
                
        bars.on("pointerover", function (event, d) {
            select(this).style("cursor", "pointer");
        })
        .on("pointerdown", function (event, d)  {
            bars.attr('filter', null)
            select(this).attr('filter', "url(#glow)");

        });

        bgRect .on("pointerdown", function (event, d)  {
            bars.attr('filter', null)

        });
                
              

            
         
            
             
              
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


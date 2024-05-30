import { axisBottom, axisLeft } from "d3-axis";
import { } from "d3-color";
import { format } from "d3-format";
import {
    ScaleBand,
    ScaleLinear,
    ScaleOrdinal,
    scaleBand,
    scaleLinear,
    scaleOrdinal,
} from "d3-scale";
import { Selection, pointer, select } from "d3-selection";
import { Series, SeriesPoint, stack } from "d3-shape";
import { html, render } from "lit-html";
import { StyleInfo, styleMap } from "lit-html/directives/style-map.js";
import { Observable, fromEvent } from "rxjs";
import { debounceTime, filter } from "rxjs/operators";
import { Ord } from "./ord";
import pattern from "./textures";
import { ChartSize, Margin } from "./types";



function toKebabCase(str: string): string {
	return str
		.replace(/([a-z])([A-Z])/g, "$1-$2") // Add hyphen between lowercase and uppercase characters
		.replace(/[\s_]+/g, "-") // Replace spaces and underscores with hyphens
		.toLowerCase(); // Convert the entire string to lowercase
}

const blues = [
	"#7ea8cb",
	"#8EC1D3",
	"#9FD6DB",
	"#AFE2DD",
	"#C0E9DD",
	"#D2F0E2",
	"#E4F6EA",
];

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
	"#FF338D", // Magenta
];


type Range = { lower: number, higher: number}
type ComborbidityKind = 'PSYCHIATRIC'|'MEDICAL'
interface PtsdComorbidities {
    name: string,
    kind: ComborbidityKind,
    comorbidity_percentage_range: Range | null,
    risk_multiplier_range: Range | null,
    explaination?: string;
}

export const comorbidities: Array<PtsdComorbidities> = [
    {
        name: 'Substance use',
        kind: 'PSYCHIATRIC',
        comorbidity_percentage_range: { lower: 46, higher: 46},
        risk_multiplier_range: null
    },
    {
        name: 'Alcohol use',
        kind: 'PSYCHIATRIC',
        comorbidity_percentage_range: { lower: 10, higher: 10},
        risk_multiplier_range: null
    },
    {
        name: 'Major Depressive Disorder (MDD)',
        kind: 'PSYCHIATRIC',
        comorbidity_percentage_range: { lower: 50, higher: 50},
        risk_multiplier_range: null
    },
    {
        name: 'Anxiety',
        kind: 'PSYCHIATRIC',
        comorbidity_percentage_range: { lower: 10, higher: 10},
        risk_multiplier_range: null
    },
    {
        name: 'Self-harm',
        kind: 'PSYCHIATRIC',
        comorbidity_percentage_range: { lower: 5, higher: 19},
        risk_multiplier_range: null
    },
    {
        name: 'Chronic pain',
        kind: 'MEDICAL',
        comorbidity_percentage_range: { lower: 20, higher: 20},
        risk_multiplier_range: null
    },
    {
        name: 'Inflammation',
        kind: 'MEDICAL',
        comorbidity_percentage_range: { lower: 51, higher: 51},
        risk_multiplier_range: { lower: 2, higher: 2}
    },
    {
        name: 'Cardiometabolic disorders',
        kind: 'MEDICAL',
        comorbidity_percentage_range: null,
        risk_multiplier_range: { lower: 1.27, higher: 1.53}
    },
    {
        name: 'Dementia',
        kind: 'MEDICAL',
        comorbidity_percentage_range: null,
        risk_multiplier_range: { lower: 1.61, higher: 1.61}
    },
    {
        name: 'Sleep dysfunction',
        kind: 'MEDICAL',
        comorbidity_percentage_range: { lower: 70, higher: 87},
        risk_multiplier_range: null
    },
]





const tooltipRenderFn = (p: SeriesPoint<PtsdComorbidities>) => {

    const range = p.data.comorbidity_percentage_range; 

    return html`<div class="relative">
    <div class="absolute left-1/2 transform -translate-x-1/2 mt-2 w-32 bg-gray-800 text-white text-center text-sm rounded-lg py-2 px-3   transition-opacity duration-300">
    <span>${0}<span class="px-2">-</span>${0}%</span></span>
    </div>
  </div>`
};

const contentRenderFn = (p: SeriesPoint<PtsdComorbidities>, colorScale: ScaleOrdinal<string, unknown, never> ) => {

    const spanStyle: Readonly<StyleInfo> = {
        backgroundColor: colorScale(p.data.name) as string,
      }; 

    return html`
    <div class="grid grid-cols-2 gap-8">
    <div>
        <div><span class="text-3xl">
                    <svg preserveAspectRatio="xMidYMid meet" width="1em" height="1em" viewBox="0 0 15 15" fill="none" xmlns="http://www.w3.org/2000/svg"><path d="M11.5 1C11.7761 1 12 1.22386 12 1.5V13.5C12 13.7761 11.7761 14 11.5 14C11.2239 14 11 13.7761 11 13.5V1.5C11 1.22386 11.2239 1 11.5 1ZM9.5 3C9.77614 3 10 3.22386 10 3.5V13.5C10 13.7761 9.77614 14 9.5 14C9.22386 14 9 13.7761 9 13.5V3.5C9 3.22386 9.22386 3 9.5 3ZM13.5 3C13.7761 3 14 3.22386 14 3.5V13.5C14 13.7761 13.7761 14 13.5 14C13.2239 14 13 13.7761 13 13.5V3.5C13 3.22386 13.2239 3 13.5 3ZM5.5 4C5.77614 4 6 4.22386 6 4.5V13.5C6 13.7761 5.77614 14 5.5 14C5.22386 14 5 13.7761 5 13.5V4.5C5 4.22386 5.22386 4 5.5 4ZM1.5 5C1.77614 5 2 5.22386 2 5.5V13.5C2 13.7761 1.77614 14 1.5 14C1.22386 14 1 13.7761 1 13.5V5.5C1 5.22386 1.22386 5 1.5 5ZM7.5 5C7.77614 5 8 5.22386 8 5.5V13.5C8 13.7761 7.77614 14 7.5 14C7.22386 14 7 13.7761 7 13.5V5.5C7 5.22386 7.22386 5 7.5 5ZM3.5 7C3.77614 7 4 7.22386 4 7.5V13.5C4 13.7761 3.77614 14 3.5 14C3.22386 14 3 13.7761 3 13.5V7.5C3 7.22386 3.22386 7 3.5 7Z" fill="currentColor" fill-rule="evenodd" clip-rule="evenodd"></path></svg>
                    <span>${0}<span class="px-2">&#177;</span>${0}%</span></span></div>
            <div class="mt-4"><span>of patients with currently active PTSD and past PTSD had <span style=${styleMap(spanStyle)} class='text-white rounded-sm px-1'>${p.data.name}</span> as a comorbidity</span></div>
        </div>
        <div>
            <div><span class="text-3xl"><svg focusable="false" preserveAspectRatio="xMidYMid meet"
                        xmlns="http://www.w3.org/2000/svg" fill="currentColor" width="1em" height="1em" viewBox="0 0 16 16"
                        aria-hidden="true" class="inline">
                        <path d="M3.7 6.7L7.5 2.9 7.5 15 8.5 15 8.5 2.9 12.3 6.7 13 6 8 1 3 6z"></path>
                    </svg><span>200%</span></span></div>
            <div class="mt-4"><span>Increased risk</span></div>
        </div>
       <div>
        ${p.data.explaination}
    </div>
    </div>`
};

/**
 * https://riccardoscalco.it/textures/
 */




class StackedBarChart<
	T extends PtsdComorbidities,
> {
	private margin: Margin;
	private series: PtsdComorbidities[];
	private svg: Selection<SVGSVGElement, unknown, null, any>;
	private scaleY: ScaleBand<string>;
	private scaleX: ScaleLinear<number, number, never>;
	private colorScale: ScaleOrdinal<string, unknown, never>;
	private size: ChartSize;
	private resizeObservable$: Observable<Event>;
  

	private parentContainer: HTMLElement;
	private graphContainer: HTMLElement;
	private tooltipContainer: HTMLDivElement;
	private contextContainer: HTMLDivElement;

	constructor(
		data: T[],
		parentContainer: HTMLElement,
		ord: Ord<T>,
		margin: Margin = { top: 20, right: 30, bottom: 40, left: 200 },
	) {
		const sortedData = data.sort(ord.compare);
		this.series = sortedData;

		this.margin = margin;

		parentContainer.classList.add("flex", "flex-col", "p-4", "bg-black-900");

		this.parentContainer = parentContainer;

		const graphContainer = document.createElement("div");
		graphContainer.classList.add(
			"w-full",
			"h-full",
			"relative",
			"flex",
			"flex-col",
			"items-center",
			"justify-center",
		);
		this.parentContainer.append(graphContainer);
		this.graphContainer = graphContainer;

		const contextContainer = document.createElement("div");
		contextContainer.classList.add("w-full");
		this.parentContainer.append(contextContainer);
		this.contextContainer = contextContainer;

		const tooltipContainer = document.createElement("div");
		tooltipContainer.classList.add("absolute", "z-5");
		this.graphContainer.append(tooltipContainer);
		this.tooltipContainer = tooltipContainer;

		this.size = this.calculateSize(this.graphContainer, margin);

		const names = this.series.map((d) => d.name);
       

		this.scaleY = scaleBand()
			.domain(names)
			.padding(0.1)
			.range([0, this.size.innerHeight]);

		// const xs = data.flatMap((datum) =>
		// 	Object.values(datum.stack).reduce((sum, x) => sum + x.value, 0),
		// );

		this.scaleX = scaleLinear()
			// .domain([0, Math.max(...xs)])
			.domain([0, 100])
			.nice()
			.range([0, this.size.innerWidth]);
        

		const colorScale = scaleOrdinal().domain(names).range(colors);

		this.colorScale = colorScale;

       

		const svg = select(this.graphContainer).append("svg");

		svg
			.attr("width", "100%")
			.attr("height", "100%")
			.attr(
				"viewport",
				`${0} ${0} ${this.size.innerWidth} ${this.size.innerHeight}`,
			)
			.attr("data-interactive", true);

		this.svg = svg;

		this.onResize();
		this.draw();

		this.resizeObservable$ = fromEvent(window, "resize").pipe(
			debounceTime(200),
		);
		this.resizeObservable$.subscribe(() => {
			this.onResize();
		});

		const clickOutside$ = fromEvent(document, "click").pipe(
			filter((event) => {
				// Check if the click event target is outside the 'myDiv' element
				return !this.graphContainer.contains(event.target);
			}),
		);

		clickOutside$.subscribe(() => {
			select(this.tooltipContainer).transition().style("opacity", 0);
		});
	}

	private draw(): void {
        const tooltipContainer = this.tooltipContainer;
		const graphContainer = this.graphContainer;
		const contextContainer = this.contextContainer;
        const colorScaleFn = this.colorScale


		this.svg.selectAll("*").remove();

		const defs = this.svg.append("defs");

		const flowFilter = defs.append("filter").attr("id", "glow");
		flowFilter
			.append("feGaussianBlur")
			.attr("stdDeviation", "3.5")
			.attr("result", "coloredBlur");
		const feMerge = flowFilter.append("feMerge");
		feMerge.append("feMergeNode").attr("in", "coloredBlur");
		feMerge.append("feMergeNode").attr("in", "SourceGraphic");

		const pattern1 = pattern.lines().size(8).strokeWidth(2).stroke("#7ea8cb");

		this.svg.call(pattern1);

		const vizLayer = this.svg
			.append("g")
			.attr("transform", `translate(${this.margin.left},${this.margin.top})`);
		vizLayer.classed("viz");

		const graphLayer = vizLayer.append("g");
		graphLayer.classed("graphLayer", true);

		const bgRect = graphLayer.append("rect");
		bgRect
			.attr("data-interactive", true)
			.attr("width", this.size.innerWidth)
			.attr("height", this.size.innerHeight)
			.attr("fill", "transparent");



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

		const xAxisContainer = vizLayer.append("g");
		const xAxis = axisBottom(this.scaleX).tickFormat((n) =>
			format(".0%")(n.valueOf() / 100),
		);
		xAxisContainer
			.attr("class", "x-axis")
			.attr("transform", `translate(0,${this.size.innerHeight})`)
			.call(xAxis);

		const yAxis = axisLeft(this.scaleY).ticks(10, "s");
		const yAxisContainer = vizLayer.append("g");
		yAxisContainer.attr("class", "y-axis").call(yAxis);



		const bars = graphLayer
			.selectAll("rect")
			.data(this.series)
			.join("rect")
			.attr("data-interactive", true)
			.attr("x", (d) => this.scaleX(d.))
			.attr("y", (d) => this.scaleY(d.data.stackId as unknown as string)!)
           
			.attr("height", (d) => this.scaleY.bandwidth())
			.attr("mask", (d) => `url(#${toKebabCase(`mask-${d.data.stackId}`)})`)
            .style("fill", (d) => { 
                console.log(d)
                return colorScaleFn(d.data.stackId) as string
             } )

        bars.transition()
        .attr("width", (d) => this.scaleX(d[1]) - this.scaleX(d[0]));

	

		bars.on("pointerover", function (event, d) {
			select(this).style("cursor", "pointer");
		});
		bars
			.on("pointerout", function (event, d) {})
			.on("pointerdown", function (event, d) {
				bars.attr("filter", null);
				const bar = select(this);
				bar.attr("filter", "url(#glow)");

				render(tooltipRenderFn(d), tooltipContainer);
				render(contentRenderFn(d, colorScaleFn), contextContainer);

				const [x, y] = pointer(event, graphContainer);

				select(tooltipContainer).style("opacity", 1);

				select(tooltipContainer)
					.transition()
					.style("top", `${y}px`)
					.style("left", `${x}px`);
			})
			.on("pointermove", function (e, d) {});

		bgRect.on("pointerdown", function (event, d) {
			bars.attr("filter", null);
			select(tooltipContainer).transition().style("opacity", 0);
		});
	}

	private onResize(): void {
		this.size = this.calculateSize(this.graphContainer, this.margin);

		this.scaleX.range([0, this.size.innerWidth]);
		this.scaleY.range([this.size.innerHeight, 0]);

		this.svg
			.attr("width", "100%")
			.attr("height", "100%")
			.attr(
				"viewport",
				`"${0} ${0} ${this.size.innerWidth} ${this.size.innerHeight}"`,
			);

		this.draw();
	}

	private calculateSize(containerEl: HTMLElement, margin: Margin): ChartSize {
		const rect = containerEl.getBoundingClientRect();
		const innerWidth = rect.width - margin.left - margin.right;
		const innerHeight = rect.height - margin.top - margin.bottom;

		return {
			innerWidth,
			innerHeight,
			outerWidth: rect.width,
			outerHeight: rect.height,
		};
	}
}

const psychiatricComorbidities: Array<StackedBarChartData<ComborbidityDatum<number>>> = [
	{
		stackId: "Substance use",
        meta: {},
		stack: {
			baseline: 46,
			uncertainty: 0
		},
	},
	{
		stackId: "Alcohol use",
        meta: {},
		stack: {
			baseline: 10,
			uncertainty: 0
		},
	},
	{
		stackId: "Major Depressive Disorder",
        meta: {},
		stack: {
			baseline: 50,
			uncertainty: 0
		},
	},
    {
		stackId: "Anxiety",
        meta: {},
		stack: {
			baseline: 5,
			uncertainty: 14
		},
	},
];






const barChartOrd: Ord<StackedBarChartData<ComborbidityDatum<number>>> = {
	equals: (x, y) => x === y,
	compare: (first, second) => {

        const sum = (stack: Record<string, StackItem>) => {
            Object.values(stack).reduce((sum, x) => sum + x.value, 0)
        }
        
        return sum(first.stack) > sum(second.stack) ? 1 : sum(first.stack) < sum(second.stack) ? -1 : 0
    }
};

export { StackedBarChart, barChartOrd, medicalCormidities };


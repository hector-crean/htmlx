import { ComborbidityKind, PtsdComorbidities } from "@/data/comborbidities";
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
import { curveCatmullRom, line } from "d3-shape";
import { html, nothing, render } from "lit-html";
import { StyleInfo, styleMap } from "lit-html/directives/style-map.js";
import { unsafeHTML } from "lit-html/directives/unsafe-html.js";
import { when } from "lit-html/directives/when.js";
import { Observable, fromEvent } from "rxjs";
import { debounceTime, filter } from "rxjs/operators";
import { Ord } from "./ord";
import pattern from "./textures";
import { ChartSize, Margin } from "./types";

import { icons } from "@/assets/icons";

const iconsMap = (comorbidity: string) => {
	switch(comorbidity){
		default:
			return icons.AvoidDistressingThoughts
	}
}


function toKebabCase(str: string): string {
	return str
		.replace(/([a-z])([A-Z])/g, "$1-$2") // Add hyphen between lowercase and uppercase characters
		.replace(/[\s_]+/g, "-") // Replace spaces and underscores with hyphens
		.toLowerCase(); // Convert the entire string to lowercase
}

const blues = [
	"#7438C3",
	"#3576C6",
	"#CC91D3",
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


type Range = { lower: number, higher: number }




const tooltipRenderFn = (p: PtsdComorbidities) => {


	return html`<div class="relative">
    <div class="absolute transform -translate-y-full mt-2 w-32 bg-gray-800 text-white text-center text-sm py-2 px-3   transition-opacity duration-300 rounded-tl-lg rounded-tr-lg rounded-br-lg ">
    ${when(p.comorbidity_percentage_lower !== p.comorbidity_percentage_higher,
		() => html`<span>${p.comorbidity_percentage_lower}<span class="px-2">-</span>${p.comorbidity_percentage_higher}%</span></span>`,
		() => html`<span>${p.comorbidity_percentage_lower}%</span></span>`
	)}
    </div>
  </div>`
};

const contentRenderFn = (p: PtsdComorbidities, colorScale: ScaleOrdinal<string, unknown, never>) => {

	const spanStyle: Readonly<StyleInfo> = {
		backgroundColor: colorScale(p.name) as string,
	};
	const headingStyle: Readonly<StyleInfo> = {
		color: colorScale(p.name) as string,
		backgroundColor: '#374151'
	};

	const infoStyle: Readonly<StyleInfo> = {
		'border-color': colorScale(p.name) as string,
	};


	const cross = html`<span class='px-2'><svg width="15" height="15" viewBox="0 0 15 15" fill="none" xmlns="http://www.w3.org/2000/svg"><path d="M11.7816 4.03157C12.0062 3.80702 12.0062 3.44295 11.7816 3.2184C11.5571 2.99385 11.193 2.99385 10.9685 3.2184L7.50005 6.68682L4.03164 3.2184C3.80708 2.99385 3.44301 2.99385 3.21846 3.2184C2.99391 3.44295 2.99391 3.80702 3.21846 4.03157L6.68688 7.49999L3.21846 10.9684C2.99391 11.193 2.99391 11.557 3.21846 11.7816C3.44301 12.0061 3.80708 12.0061 4.03164 11.7816L7.50005 8.31316L10.9685 11.7816C11.193 12.0061 11.5571 12.0061 11.7816 11.7816C12.0062 11.557 12.0062 11.193 11.7816 10.9684L8.31322 7.49999L11.7816 4.03157Z" fill="currentColor" fill-rule="evenodd" clip-rule="evenodd"></path></svg></span>`


	const riskRendered = when(p?.comorbidity_percentage_lower !== p?.comorbidity_percentage_higher,
		() => html`<span class="text-3xl">
		<svg focusable="false" preserveAspectRatio="xMidYMid meet"
					xmlns="http://www.w3.org/2000/svg" fill="currentColor" width="1em" height="1em" viewBox="0 0 16 16"
					aria-hidden="true" class="inline">
					<path d="M3.7 6.7L7.5 2.9 7.5 15 8.5 15 8.5 2.9 12.3 6.7 13 6 8 1 3 6z"></path>
		</svg><span>${p.risk_multiplier_lower}<span class="px-2">-</span>${p.risk_multiplier_higher}${cross}</span></span></span>`,
		() => html`<span class="text-3xl">
		<svg focusable="false" preserveAspectRatio="xMidYMid meet"
					xmlns="http://www.w3.org/2000/svg" fill="currentColor" width="1em" height="1em" viewBox="0 0 16 16"
					aria-hidden="true" class="inline">
					<path d="M3.7 6.7L7.5 2.9 7.5 15 8.5 15 8.5 2.9 12.3 6.7 13 6 8 1 3 6z"></path>
		</svg><span>${p.risk_multiplier_lower}${cross}</span></span></span>`
	)

	const headlineRendered = (p: PtsdComorbidities) => html`
		<span class="text-3xl">
		    <svg focusable="false" preserveAspectRatio="xMidYMid meet" xmlns="http://www.w3.org/2000/svg" fill="currentColor"
		        width="1em" height="1em" viewBox="0 0 15 15" aria-hidden="true" class="inline">
		        <path
		            d="M7.49991 0.876892C3.84222 0.876892 0.877075 3.84204 0.877075 7.49972C0.877075 11.1574 3.84222 14.1226 7.49991 14.1226C11.1576 14.1226 14.1227 11.1574 14.1227 7.49972C14.1227 3.84204 11.1576 0.876892 7.49991 0.876892ZM1.82707 7.49972C1.82707 4.36671 4.36689 1.82689 7.49991 1.82689C10.6329 1.82689 13.1727 4.36671 13.1727 7.49972C13.1727 10.6327 10.6329 13.1726 7.49991 13.1726C4.36689 13.1726 1.82707 10.6327 1.82707 7.49972ZM8.24992 4.49999C8.24992 4.9142 7.91413 5.24999 7.49992 5.24999C7.08571 5.24999 6.74992 4.9142 6.74992 4.49999C6.74992 4.08577 7.08571 3.74999 7.49992 3.74999C7.91413 3.74999 8.24992 4.08577 8.24992 4.49999ZM6.00003 5.99999H6.50003H7.50003C7.77618 5.99999 8.00003 6.22384 8.00003 6.49999V9.99999H8.50003H9.00003V11H8.50003H7.50003H6.50003H6.00003V9.99999H6.50003H7.00003V6.99999H6.50003H6.00003V5.99999Z"
		            fill="currentColor" fill-rule="evenodd" clip-rule="evenodd"></path>
		    </svg>
		    <span class="text-lg">${p.kind}</span>
		</span>`

	const comorbidityText = (comorb: PtsdComorbidities) => {

		
		return html`${when(
			['Dementia', 'Cardiometabolic disorders', 'Inflammation'].includes(comorb.name), 
			() => html`<span> increased risk of developing <span style=${styleMap(spanStyle)}
			class='px-1 text-white rounded-sm'>${comorb.name}</span>, if diagnosed with PTSD</span>`,
			() => html`<span>of patients with PTSD had <span style=${styleMap(spanStyle)}
			class='px-1 text-white rounded-sm'>${comorb.name}</span> as a comorbidity</span>`
		)
		}`
	}


	return html`
<div class="flex flex-col gap-4 rounded-lg bg-[#e6f0f6] p-8">


<div class="col-span-1">
	<div>
		<span class="text-3xl">
			<svg preserveAspectRatio="xMidYMid meet" width="1em" height="1em" viewBox="0 0 15 15" fill="none"
				xmlns="http://www.w3.org/2000/svg">
				<path
					d="M11.5 1C11.7761 1 12 1.22386 12 1.5V13.5C12 13.7761 11.7761 14 11.5 14C11.2239 14 11 13.7761 11 13.5V1.5C11 1.22386 11.2239 1 11.5 1ZM9.5 3C9.77614 3 10 3.22386 10 3.5V13.5C10 13.7761 9.77614 14 9.5 14C9.22386 14 9 13.7761 9 13.5V3.5C9 3.22386 9.22386 3 9.5 3ZM13.5 3C13.7761 3 14 3.22386 14 3.5V13.5C14 13.7761 13.7761 14 13.5 14C13.2239 14 13 13.7761 13 13.5V3.5C13 3.22386 13.2239 3 13.5 3ZM5.5 4C5.77614 4 6 4.22386 6 4.5V13.5C6 13.7761 5.77614 14 5.5 14C5.22386 14 5 13.7761 5 13.5V4.5C5 4.22386 5.22386 4 5.5 4ZM1.5 5C1.77614 5 2 5.22386 2 5.5V13.5C2 13.7761 1.77614 14 1.5 14C1.22386 14 1 13.7761 1 13.5V5.5C1 5.22386 1.22386 5 1.5 5ZM7.5 5C7.77614 5 8 5.22386 8 5.5V13.5C8 13.7761 7.77614 14 7.5 14C7.22386 14 7 13.7761 7 13.5V5.5C7 5.22386 7.22386 5 7.5 5ZM3.5 7C3.77614 7 4 7.22386 4 7.5V13.5C4 13.7761 3.77614 14 3.5 14C3.22386 14 3 13.7761 3 13.5V7.5C3 7.22386 3.22386 7 3.5 7Z"
					fill="currentColor" fill-rule="evenodd" clip-rule="evenodd"></path>
			</svg>
			${when(p.comorbidity_percentage_lower !== p.comorbidity_percentage_higher,
			() => html`<span>${p.comorbidity_percentage_lower === null ? 'unknown' :
				p.comorbidity_percentage_lower}<span
					class="px-2">-</span>${p.comorbidity_percentage_higher}%</span></span>`,
		() => html`<span>${p.comorbidity_percentage_lower}%</span></span>`
		)}
		</span>
	</div>
	<div class="mt-4">${comorbidityText(p)}</div>
</div>

<div class="col-span-1">
	<div>
		${when(Boolean(p.risk_multiplier_lower) && Boolean(p.risk_multiplier_higher), () => riskRendered, () =>
		nothing)}
	</div>
	${when(Boolean(p.risk_multiplier_lower) && Boolean(p.risk_multiplier_higher), () => html`<div class="mt-4">
		<span> increased risk of developing <span style=${styleMap(spanStyle)}
				class='px-1 text-white rounded-sm'>${p.name}</span>, if diagnosed with PTSD</span>`, () =>
		nothing)}
	</div>

	<!-- <div class="col-span-1">
		${headlineRendered(p)}
		
	</div> -->



	<div class='flex flex-col items-center justify-center col-span-1'>
		<div class='pl-2 border-l-4' style=${styleMap(infoStyle)}>${unsafeHTML(p.explanation)}</div>
	</div>
</div>`
};

/**
 * https://riccardoscalco.it/textures/
 */




class BarChart<
	T extends PtsdComorbidities,
> {
	private kindSelected: Array<ComborbidityKind>;
	private margin: Margin;
	private series: PtsdComorbidities[];
	private filteredSeries: PtsdComorbidities[]
	private svg: Selection<SVGSVGElement, unknown, null, any>;
	private scaleY: ScaleBand<string>;
	private scaleX: ScaleLinear<number, number, never>;
	private colorScale: ScaleOrdinal<string, unknown, never>;
	private combirbidityTypeColorScale: ScaleOrdinal<string, unknown, never>;
	private size: ChartSize;
	private resizeObservable$: Observable<Event>;
	private iconsMap: (comorbidity: string) => string; 


	private parentContainer: HTMLElement;
	private graphContainer: HTMLElement;
	private tooltipContainer: HTMLDivElement;
	private contextContainer: HTMLDivElement;
	private filterContainer: HTMLElement;

	constructor(
		data: T[],
	parentContainer: HTMLElement,
		ord: Ord<T>,
		margin: Margin = { top: 50, right: 30, bottom: 100, left: 200 },
	) {
		
		this.iconsMap = iconsMap;
		
		const sortedData = data.sort(ord.compare);
		this.kindSelected = ['MEDICAL', 'PSYCHIATRIC'];
		this.series = sortedData;
		this.filteredSeries = this.series.filter(d => this.kindSelected.includes(d.kind));

		this.margin = margin;

		parentContainer.classList.add("grid", "grid-cols-1", "grid-rows-[min-content_400px_min-content]", "p-4", "bg-black-900", "items-start", 'justify-between', 'bg-[#d4e4ee]', 'rounded-lg', 'mt-2', 'mx-2');

		this.parentContainer = parentContainer;

		const filterContainer = document.createElement('div');
		filterContainer.classList.add('flex', 'flex-row', 'items-start', 'justify-start', 'gap-4')
		this.parentContainer.append(filterContainer);
		this.filterContainer = filterContainer

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
		contextContainer.classList.add("presentation_wrapper");
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
		const xs = data.map(d => d.comorbidity_percentage_lower ?? 0)

		this.scaleX = scaleLinear()
			.domain([0, 100])
			.nice()
			.range([0, this.size.innerWidth]);


		const colorScale = scaleOrdinal().domain(names).range(colors);

		const comborbidityKinds: Array<ComborbidityKind> = ['MEDICAL', 'PSYCHIATRIC']
		const combirbidityTypeColorScale = scaleOrdinal().domain(comborbidityKinds).range(blues);
		this.combirbidityTypeColorScale = combirbidityTypeColorScale;

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
		const combirbidityTypeColorScale = this.combirbidityTypeColorScale


		const comborbidityKinds: Array<ComborbidityKind> = ['MEDICAL', 'PSYCHIATRIC']
		const filterContainerSelection = select(this.filterContainer);
		filterContainerSelection.selectAll("*").remove();


		const btns = filterContainerSelection.selectAll("div")
			.data(comborbidityKinds)
			.enter()
			.append("div")
			.attr('class', 'text-white rounded-lg px-4 py-2 cursor-pointer shadow-lg hover:bg-blue-500 transition-all duration-200')
			.style('background-color', d => this.combirbidityTypeColorScale(d)!)
			.style('opacity', d => this.kindSelected.includes(d) ? 1 : 0.5)
			.html(d => d)
			.on('pointerdown', (event, d) => {
				if (this.kindSelected.includes(d)) {
					this.kindSelected = this.kindSelected.filter(x => x !== d);
				} else {
					this.kindSelected = [...new Set([...this.kindSelected, d])]
				}


				this.filteredSeries = this.series.filter(d => this.kindSelected.includes(d.kind));

				this.draw()
			})





		this.svg.selectAll("*").remove();

		const defs = this.svg.append("defs");


		const markerUnit = 10;
		const markerBoxWidth = markerUnit;
		const markerBoxHeight = markerUnit;
		const refX = markerUnit;
		const refY = markerBoxWidth / 2;
		const markerWidth = markerBoxHeight / 2;
		const markerHeight = markerBoxHeight / 2;
		const arrowPoints = [[0, 0], [0, markerUnit], [markerUnit * 2, markerUnit]];

		defs.append('marker')
			.attr('id', 'arrow')
			.attr('viewBox', [0, 0, markerBoxWidth, markerBoxHeight])
			.attr('refX', refX)
			.attr('refY', refY)
			.attr('markerWidth', markerBoxWidth)
			.attr('markerHeight', markerBoxHeight)
			.attr('orient', 'auto-start-reverse')
			.append('path')
			.attr('d', line()(arrowPoints))
			.attr('stroke', 'black');

		const flowFilter = defs.append("filter").attr("id", "glow");
		flowFilter
			.append("feGaussianBlur")
			.attr("stdDeviation", "3.5")
			.attr("result", "coloredBlur");
		const feMerge = flowFilter.append("feMerge");
		feMerge.append("feMergeNode").attr("in", "coloredBlur");
		feMerge.append("feMergeNode").attr("in", "SourceGraphic");

		const errorPattern = pattern.lines().size(8).strokeWidth(2);
		const circlePattern = pattern.circles().size(5).complement().lighter()

		this.svg.call(errorPattern);
		this.svg.call(circlePattern);


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



		const touchBgLayer = graphLayer.append('g')


		const upperBoundLayer = graphLayer.append('g')

		const lowerBoundLayer = graphLayer.append('g');

		const placeholderLayer = graphLayer.append('g');

		const annotationsLayer = graphLayer.append('g')






		const bars = lowerBoundLayer
			.selectAll("path")
			.data(this.filteredSeries)
			.join("path")
			.attr("data-interactive", true)
			.attr("d", (d) =>
                RightRoundedRect(
                    this.scaleX(0),
                    this.scaleY(d.name)!,
                    this.scaleX(0),
                    this.scaleY.bandwidth(),
                    this.scaleY.bandwidth()/2,
                ),
            )
			.attr("mask", (d) => `url(#${toKebabCase(`mask-${d.name}`)})`)
			.style("fill", (d) => {
				return combirbidityTypeColorScale(d.kind) as string
			})
			.style('pointer-events', 'none'); // Add this line to disable pointer and touch events


		bars.transition()
			.attr("d", (d) =>
                RightRoundedRect(
					this.scaleX(0),
                    this.scaleY(d.name)!,
                    this.scaleX(d.comorbidity_percentage_lower ?? 0),
                    this.scaleY.bandwidth(),
                    this.scaleY.bandwidth()/2,
                ),
            );


		const errorMarginBars = upperBoundLayer
			.selectAll("path")
			.data(this.filteredSeries)
			.join("path")
			.attr("data-interactive", true)
			.attr("d", (d) =>
                RightRoundedRect(
                    this.scaleX(0),
                    this.scaleY(d.name)!,
                    this.scaleX(0),
                    this.scaleY.bandwidth(),
                    this.scaleY.bandwidth()/2,
                ),
            )
			.attr("mask", (d) => `url(#${toKebabCase(`mask-${d.name}`)})`)
			.style("fill", (d) => {
				return errorPattern.url()
				// return combirbidityTypeColorScale(d.kind) as string
			})
			.style("stroke", (d) => {
				// return errorPattern.url()
				return combirbidityTypeColorScale(d.kind) as string
			})
			.style('stroke-width', 2)
			.style('pointer-events', 'none'); // Add this line to disable pointer and touch events


		errorMarginBars.transition()
			.attr("d", (d) =>
                RightRoundedRect(
					this.scaleX(0),
                    this.scaleY(d.name)!,
                    this.scaleX(d.comorbidity_percentage_higher ?? 0),
                    this.scaleY.bandwidth(),
                    this.scaleY.bandwidth()/2,
                ),
            );


		const placeholderBars = placeholderLayer
			.selectAll("rect")
			.data(this.filteredSeries.filter(d => d.comorbidity_percentage_lower === null))
			.join("rect")
			.attr("data-interactive", true)
			.attr("x", (d) => this.scaleX(0))
			.attr("y", (d) => this.scaleY(d.name)!)
			.attr("height", (d) => this.scaleY.bandwidth())
			.attr("mask", (d) => `url(#${toKebabCase(`mask-${d.name}`)})`)
			.style("fill", (d) => {
				return circlePattern.url()
			})
			.style('opacity', 0.8)
			.style('pointer-events', 'none'); // Add this line to disable pointer and touch events


		placeholderBars.transition()
			.attr("width", (d) => this.scaleX(100))



		errorMarginBars.on("pointerover", function (event, d) {
			select(this).style("cursor", "pointer");
		});

		errorMarginBars
			.on("pointerdown", function (event, d) {
				bars.attr("filter", null);
				const bar = select(this);
				bar.attr("filter", "url(#glow)");

				render(contentRenderFn(d, colorScaleFn), contextContainer);

				const [x, y] = pointer(event, graphContainer);

				select(tooltipContainer).style("opacity", 1);

				select(tooltipContainer)
					.transition()
					.style("top", `${y}px`)
					.style("left", `${x}px`);
			})


		const touchBgRects = touchBgLayer
			.selectAll("rect")
			.data(this.filteredSeries)
			.join("rect")
			.attr("data-interactive", true)
			.attr("x", (d) => -this.margin.left)
			.attr("y", (d) => this.scaleY(d.name)!)
			.attr("height", (d) => this.scaleY.bandwidth())
			.style("fill", (d) => {
				return colorScaleFn(d.name) as string
			})
			.attr('opacity', 0.1);

		touchBgRects.transition()
			.attr("width", (d) => this.size.innerWidth + this.margin.left)

			// touchBgLayer
			// .selectAll("image")
			// .data(this.filteredSeries)
			// .join("image")
			// .attr("data-interactive", true)
			// .attr("xlink:href", (d) => this.iconsMap(d.name))
			// .attr("width", this.scaleY.bandwidth())
			// .attr("height", this.scaleY.bandwidth())
			// .attr("x", -this.margin.left)
			// .attr("y", (d) => this.scaleY(d.name)!)



		touchBgRects.on("pointerover", function (event, d) {
			select(this).style("cursor", "pointer");
		});

		touchBgRects
			.on("pointerdown", function (event, d) {
				touchBgRects.attr("opacity", 0.1);

				const touchBgRect = select(this);
				touchBgRect.attr('opacity', 1)



				select(tooltipContainer).transition().style("opacity", 0);
				render(contentRenderFn(d, colorScaleFn), contextContainer);




			})


		bars.on("pointerover", function (event, d) {
			select(this).style("cursor", "pointer");
		});
		bars
			.on("pointerout", function (event, d) { })
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
			.on("pointermove", function (e, d) { });

		bgRect.on("pointerdown", function (event, d) {
			bars.attr("filter", null);
			select(tooltipContainer).transition().style("opacity", 0);
		});



		const annotationLine = line<[number, number]>()
			.x(d => d[0])
			.y(d => d[1])
			.curve(curveCatmullRom.alpha(0.5));





		// No Data annotation
		// const noDataPathData = annotationLine([
		// 	[this.scaleX(45), this.size.innerHeight - this.scaleY.bandwidth() / 2],
		// 	[this.scaleX(47), this.size.innerHeight + this.margin.bottom / 2],
		// 	[this.scaleX(54), this.size.innerHeight + this.margin.bottom / 2]
		// ]);

		// annotationsLayer
		// 	.append('path')
		// 	.attr('d', noDataPathData)
		// 	.attr('stroke-width', 1)
		// 	.attr('stroke', 'gray')
		// 	// .attr('marker-start', 'url(#arrow)')
		// 	// .attr('marker-end', 'url(#arrow)')
		// 	.attr('fill', 'transparent');

		// annotationsLayer.append('circle').attr('cx', this.scaleX(45)).attr('cy', this.size.innerHeight - this.scaleY.bandwidth() / 2).attr('r', 4).style('fill', 'white').style('stroke', 'black');
		// annotationsLayer.append('rect').attr('x', this.scaleX(54.5)).attr('y', (this.size.innerHeight + this.margin.bottom / 2 - 15)).attr('width', 30).attr('height', 30).attr('fill', circlePattern.url())
		// annotationsLayer.append('text').attr('x', this.scaleX(54.5) + 35).attr('y', (this.size.innerHeight + this.margin.bottom / 2 + 4)).text('No Data');

		// Interval annotation: 
		// const START = 12;
		// const MIDPOINT = 14;
		// const END = 21;
		// const SQUARE_WIDTH = 30;

		// const intervalAnnotationPathData = annotationLine([
		// 	[this.scaleX(START), this.size.innerHeight - 3 * this.scaleY.bandwidth()],
		// 	[this.scaleX(MIDPOINT), this.size.innerHeight + this.margin.bottom / 2],
		// 	[this.scaleX(END), this.size.innerHeight + this.margin.bottom / 2]
		// ]);

		// annotationsLayer
		// 	.append('path')
		// 	.attr('d', intervalAnnotationPathData)
		// 	.attr('stroke-width', 1)
		// 	.attr('stroke', 'gray')
		// 	// .attr('marker-start', 'url(#arrow)')
		// 	// .attr('marker-end', 'url(#arrow)')
		// 	.attr('fill', 'transparent');

		// annotationsLayer.append('circle').attr('cx', this.scaleX(START)).attr('cy', this.size.innerHeight - 3 * this.scaleY.bandwidth()).attr('r', 4).style('fill', 'white').style('stroke', 'black');
		// annotationsLayer.append('rect').attr('x', this.scaleX(END + 0.5)).attr('y', (this.size.innerHeight + this.margin.bottom / 2 - SQUARE_WIDTH / 2)).attr('width', SQUARE_WIDTH).attr('height', SQUARE_WIDTH).attr('fill', errorPattern.url())
		// annotationsLayer.append('text').attr('x', this.scaleX(END + 0.5) + SQUARE_WIDTH + 5).attr('y', (this.size.innerHeight + this.margin.bottom / 2 + 4)).text('Uncertainty');



		const START = 5;
		const MIDPOINT = 14;
		const END = 21;
		const SQUARE_WIDTH = this.margin.top / 2



		const intervalAnnotationPathData = annotationLine([
			[this.scaleX(START), this.size.innerHeight - 3 * this.scaleY.bandwidth()],
			[this.scaleX(MIDPOINT), this.size.innerHeight + this.margin.bottom / 2],
			[this.scaleX(END), this.size.innerHeight + this.margin.bottom / 2]
		]);

		annotationsLayer
			.append('path')
			.attr('d', intervalAnnotationPathData)
			.attr('stroke-width', 1)
			.attr('stroke', 'gray')
			// .attr('marker-start', 'url(#arrow)')
			// .attr('marker-end', 'url(#arrow)')
			.attr('fill', 'transparent');

		annotationsLayer.append('circle').attr('cx', this.scaleX(START)).attr('cy', this.size.innerHeight - 3 * this.scaleY.bandwidth()).attr('r', 4).style('fill', 'white').style('stroke', 'black');
		annotationsLayer
			.append('text')
			.attr('x', this.scaleX(END + 0.5) + 5)
			.attr('y', (this.size.innerHeight + this.margin.bottom / 2 + 4))
			.text('Click on a bar to reveal more');


		const legend = annotationsLayer.append('g').attr("transform", `translate(${0},${-this.margin.top + SQUARE_WIDTH})`)



		legend.append('rect').attr('x', this.scaleX(0)).attr('y', (- SQUARE_WIDTH / 2)).attr('width', SQUARE_WIDTH).attr('height', SQUARE_WIDTH).attr('fill', errorPattern.url())
		legend.append('text').attr('x', this.scaleX(0) + SQUARE_WIDTH + 5).attr('y', (4)).text('Range');


		// legend.append('rect').attr('x', this.scaleX(50)).attr('y', (- SQUARE_WIDTH / 2)).attr('width', SQUARE_WIDTH).attr('height', SQUARE_WIDTH).attr('fill', circlePattern.url())
		// legend.append('text').attr('x', this.scaleX(50) + SQUARE_WIDTH + 5).attr('y', (+ 4)).text('No % Data');



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

const barChartOrd: Ord<PtsdComorbidities> = {
	equals: (x, y) => x === y,
	compare: (first, second) => {



		return (first.comorbidity_percentage_lower ?? 0) > (second.comorbidity_percentage_lower ?? 0) ? 1 : (first.comorbidity_percentage_lower ?? 0) < (second.comorbidity_percentage_lower ?? 0) ? -1 : 0
	}
};

export { BarChart, barChartOrd };




function RightRoundedRect(x: number, y: number, width: number, height: number, radius: number): string {
    return (
        `M${x},${y}h${(width - radius)}a${radius},${radius} 0 0 1 ${radius},${radius}v${(height - 2 * radius)}a${radius},${radius} 0 0 1 ${-radius},${radius}h${(radius - width)}z`
    );
}
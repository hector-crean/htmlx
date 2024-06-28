import { ChartSize, Margin } from "@/lib/types/chart";
import { ScaleBand, ScaleLinear, scaleBand, scaleLinear, scaleOrdinal } from 'd3-scale';
import { LitElement, SVGTemplateResult, css, html, nothing, svg } from 'lit';
import { classMap } from 'lit-html/directives/class-map.js';
import { map } from "lit-html/directives/map.js";
import { createRef, ref } from 'lit-html/directives/ref.js';
import { unsafeHTML } from "lit-html/directives/unsafe-html.js";
import { when } from "lit-html/directives/when.js";
import { property } from 'lit/decorators.js';
import { Observable, debounceTime, filter, fromEvent } from 'rxjs';

const DEFAULT_ASPECT_RATIO = 2;
const DEFAULT_HEIGHT = 540;
const BLUES = [
    "#7da7ca",
    "#3576C6", 
    "#4567C5", 
    "#5557C5", 
    "#6448C4", 
    "#7438C3",
];
const DEFAULT_SIZE = { innerHeight: DEFAULT_HEIGHT, innerWidth: DEFAULT_HEIGHT * DEFAULT_ASPECT_RATIO, outerHeight: DEFAULT_HEIGHT, outerWidth: DEFAULT_HEIGHT * DEFAULT_ASPECT_RATIO };

const DEFAULT_MARGIN: Margin = { left: 150, right: 40, top: 40, bottom: 20 }

const RAINBOW_COLORS = [
    "#FF0000", // Red
    "#FF5500", // Reddish Orange
    "#FFAA00", // Orange
    "#FFFF00", // Yellow
    "#AAFF00", // Yellowish Green
    "#55FF00", // Green
    "#00FF00", // Green
    "#00FF55", // Greenish Cyan
    "#00FFAA", // Cyan

 
];

//utility functions:
const splitText = (text: string, maxLength: number): string[] => {
    const words = text.split(' ');
    const lines: string[] = [];
    let currentLine = '';

    words.forEach((word, idx) => {
        if ((currentLine + word).length <= maxLength) {
            currentLine += (currentLine.length === 0 ? '' : ' ') + word;
        } else {
            lines.push(currentLine);
            currentLine = word;
        }
        if (idx === words.length - 1) {
            lines.push(currentLine);
        }
    });

    return lines;
};


//Render functions
const drawVerticalLine = (
    value: number | null,
    innerWidth: number,
    innerHeight: number,
    xScale: ScaleLinear<number, number, never>
): SVGTemplateResult => {
    if (value === null) return svg``;

    const xPosition = xScale(value);

    return svg`
        <line x1="${xPosition}" y1="0" x2="${xPosition}" y2="${innerHeight}" stroke="#000" stroke-dasharray="4" />
        <g transform="translate(${xPosition}, ${innerHeight - 4})">
            <rect 
                y="-12"
                x="0"
                height="14" 
                width="25"
                fill="#b9b9b9"
            ></rect>
            <text x="0" y="0" text-anchor="start" font-size="10px" fill="white">${value}%</text>
        </g>
    `;
};


const drawAxis = (
    data: Array<BarChartDatum>,
    innerWidth: number,
    innerHeight: number,
    yScale: ScaleBand<string>,
    xScale: ScaleLinear<number, number, never>
): SVGTemplateResult => {

    const drawTickLabel = (tick: string) => {
        const datum = data.find(d => d.id === tick);
        const label = datum ? datum.label : tick;
        const lines = splitText(label.trim(), 10);
        // return lines.map((line, index) => svg`
        //     <tspan x="-10" dy="${index === 0 ? 0 : 1.2}em">${line}</tspan>
        // `);
        return svg`<tspan>${label}</tspan>`
    };

    return svg`
        <g>
            <g class="x-axis" transform="translate(0, ${innerHeight})">
                <line x1="0" x2="${innerWidth}" stroke="#5d6978" stroke-width="1" ></line>
                ${xScale.ticks().map(tick => svg`
                    <g transform="translate(${xScale(tick)}, 0)">
                        <line y1="0" y2="5" stroke="#5d6978" stroke-width="1" ></line>
                        <text y="20" text-anchor="middle" font-size="10px" class="tick">${tick}</text>
                    </g>
                `)}
            </g>
            <g class="y-axis">
                <line y1="0" y2="${innerHeight}" stroke="#5d6978" stroke-width="1" ></line>
                ${yScale.domain().map(tick => svg`
                <g transform="translate(0, ${yScale(tick)! + yScale.bandwidth() / 2})">
                    <line x1="0" x2="-5" stroke="#000"></line> 
                    <text x="-10" y="0" text-anchor="end" dy="0.32em" font-size="10px" class="tick">
                        ${drawTickLabel(tick)}
                    </text>
                    </g>
                `)}
            </g>
        </g>
    `;
};

const drawLegend = (): SVGTemplateResult => {

    return svg`
        <g transform="translate(0, 0)">
            <rect 
                fill="url(#error-margin)" 
                x="0"
                y="0"
                height="20" 
                width="20"
            ></rect>
            <text x="24" y="15" text-anchor="start" font-size="10px">Range</text>
        </g>
    `
}


function rightHandedRectanglePath(x: number, y: number, width: number, height: number, radius: number): string {
    // Ensure the radius is not greater than half the width or half the height
    radius = Math.min(radius, width / 2, height / 2);

    return (
        `M${x},${y}h${width - radius}a${radius},${radius} 0 0 1 ${radius},${radius}v${height - 2 * radius}a${radius},${radius} 0 0 1 ${-radius},${radius}h${radius - width}v${-height}z`  // Close the path
    );
}


const drawBar = (
    data: Array<BarChartDatum>,
    innerWidth: number,
    innerHeight: number,
    margin: Margin,
    yScale: ScaleBand<string>,
    xScale: ScaleLinear<number, number, never>,
    hoveredBars: Set<string>,
    clickedBars: Set<string>,
    onHoverStart: (id: string) => void,
    onHoverEnd: (id: string) => void,
    onClick: (id: string) => void
): SVGTemplateResult => {

    const individualColorScale = scaleOrdinal().domain([...Array(data.length).keys()].map(n => `${n}`)).range(RAINBOW_COLORS);

    const groupColorScale = scaleOrdinal().domain([...Array(data.length).keys()].map(n => `${n}`)).range(BLUES);
    // const groupColorScale = scaleOrdinal().domain([...Array(data.length).keys()].map(n => `${n}`)).range(BLUES);

    return svg`
        <g>
            ${data.map((d, i) => {
        // const isHovered = hoveredBars.has(d.id);
        const isClicked = clickedBars.has(d.id);

        
        return svg`
                    <rect 
                        y="${yScale(d.id)}" 
                        x="${-margin.left}"
                        height="${yScale.bandwidth()}" 
                        width="${innerWidth + margin.left}"
                        fill="${isClicked ? d.fill ? d.fill : "#0b5079" : "#bdd5e8"}"
                        opacity="${isClicked ? 1 : 0.1}"
                        @click="${() => onClick(d.id)}"
                        @pointerover="${() => onHoverStart(d.id)}" 
                        @pointerout="${() => onHoverEnd(d.id)}"
                        data-interactive="true"
                    ></rect>
                    <rect 
                        stroke="red"
                        fill="url(#${d.id}-icon)"
                        y="${yScale(d.id)}" 
                        x="${-margin.left}"
                        height="${yScale.bandwidth()}" 
                        width="${yScale.bandwidth()}"
                    >
                    </rect>
                    
                    ${when(
                        typeof d.end === 'number',
                        () => svg`<path 
                        fill-rule="evenodd"
                        d="${rightHandedRectanglePath(0, yScale(d.id)!,xScale(d.end!), yScale.bandwidth(),yScale.bandwidth()/2)}"
                        fill="url(#error-margin)" 
                        stroke="${groupColorScale(`${d.groupId}`)}" 
                        stroke-width="2"
                        class="${classMap({ bar: true, clicked: isClicked })}" 
                        data-interactive="true"
                        @click="${() => onClick(d.id)}"
                        @pointerover="${() => onHoverStart(d.id)}" 
                        @pointerout="${() => onHoverEnd(d.id)}" 
                    ></path>`,
                        () => nothing
                    )}
                    <path 
                        d="${rightHandedRectanglePath(0, yScale(d.id)!,xScale(d.start), yScale.bandwidth(),yScale.bandwidth()/2)}"
                        fill="${groupColorScale(`${d.groupId}`)}" 
                        
                        class="${classMap({ bar: true, clicked: isClicked })}" 
                        data-interactive="true"
                        @click="${() => onClick(d.id)}"
                        @pointerover="${() => onHoverStart(d.id)}" 
                        @pointerout="${() => onHoverEnd(d.id)}" 

                    ></path>
                    
                    <!-- <text 
                        x="${xScale(d.start) + 5}" 
                        y="${yScale(d.id)! + yScale.bandwidth() / 2}" 
                        dy="0.32em"
                        text-anchor="start" 
                        font-size="10px" 
                        class="${classMap({ clicked: isClicked, label: true })}"
                    >
                        ${d.start}%
                    </text> -->
                `;
    })}
        </g>
    `;
};

const drawFilterButtons = (barGroupIds: Array<number>, selectedBarGroupIds: Array<number>) => {
    return html`
    <ul>
      ${map(barGroupIds, (i) => html`<li>${i}</li>`)}
    </ul>
  `;
}

export interface BarChartDatum {
    id: string,
    groupId: number,
    fill?: string,
    label: string;
    start: number;
    end?: number;
}

interface PanelElement extends HTMLElement {
    slot: string;
    id: string;
}

interface IconElement extends HTMLElement {
    slot: string;
    id: string;
}

class BarChart extends LitElement {
    @property({ type: Object }) margin: Margin =  DEFAULT_MARGIN
    @property({ type: Array }) bardata: Array<BarChartDatum> = []
    @property({ type: Number }) aspect = DEFAULT_ASPECT_RATIO;
    @property({ type: Number }) imageHeight = DEFAULT_HEIGHT;
    @property({ type: Set }) hoveredBars: Set<string> = new Set();
    @property({ type: Set }) clickedBars: Set<string> = new Set();
    @property({ type: Number }) clickedValue: number | null = null;
    @property({type: Array}) barGroupIds: Array<number>
    @property({type: Array}) selectedBarGroupIds: Array<number>

    private containerRef = createRef<HTMLDivElement>();
    private canvasRef = createRef<SVGAElement>();
    private resizeObservable$: Observable<Event>;
    private clickOutside$: Observable<Event>;
    private size: ChartSize = DEFAULT_SIZE;
    private panels: Array<PanelElement>;
    private icons: Array<IconElement>;


    constructor() {
        super();

        this.resizeObservable$ = fromEvent(window, "resize").pipe(
            debounceTime(200),
        );
        this.resizeObservable$.subscribe(async () => {
            this.onResize();
            this.requestUpdate();
        });

        this.clickOutside$ = fromEvent(document, "click").pipe(
            filter((event) => {
                // Check if the click event target is outside the 'myDiv' element
                // @ts-ignore
                return !this.canvasRef.value?.contains(event.target);
            }),
        )
        this.clickOutside$.subscribe(() => {
			// this.onBackgroundClick()
		})

        this.panels = Array.from(this.querySelectorAll("[slot=panel][id]"));
        this.icons = Array.from(this.querySelectorAll("[slot=icon][id]"));


        const groupsIds = [...new Set(this.bardata.map(bar => bar.groupId))]
        this.barGroupIds = groupsIds
        this.selectedBarGroupIds = groupsIds

        this.onResize();
    }

    

    onBarHoverStart(barId: string) {
        this.hoveredBars.add(barId)
        this.requestUpdate();
    }
    onBarHoverEnd(barId: string) {
        this.hoveredBars.delete(barId)
        this.requestUpdate();
    }

    
    onBarClick(barId: string) {
        const clickedDatum = this.bardata.find(d => d.id === barId);
        this.clickedValue = clickedDatum ? clickedDatum.start : null;
        this.clickedBars = new Set([barId]);

        this.panels.forEach((panel) => panel.removeAttribute("selected"));

        this.clickedBars.forEach(barId => {
            const panel = this.panels.find(panel => panel.id === barId);
            if(panel){
                panel.setAttribute("selected", "");
            }
        })

        this.requestUpdate();
    }

    onBackgroundClick(){
        this.clickedBars = new Set([]);
        this.clickedValue = null;
        this.panels.forEach((panel) => panel.removeAttribute("selected"));

        this.requestUpdate();
    }

    private getScales() {
        const ys = this.bardata.map(d => d.id);
        const yScale = scaleBand<string>()
            .domain(ys)
            .range([0, this.size.innerHeight])
            .padding(0.1);

        const xs = this.bardata.map(d => d.start)
        const xsEnd = this.bardata.map(d => d.end).filter(d => !!d).map(d => Number(d));
        
        const xScale = scaleLinear()
            .domain([Math.min(0, ...xs), Math.max(...xs, ...xsEnd)])
            .nice()
            .range([0, this.size.innerWidth]);

        return { yScale, xScale };
    }

    updated(changedProperties: Map<string, any>) {
        if (changedProperties.has('aspect') || changedProperties.has('imageHeight') || changedProperties.has('bardata')) {
            this.requestUpdate();
        }
    }

    firstUpdated() {
        this.onResize();
    }

    private calculateSize(containerEl: HTMLElement | undefined, margin: Margin): ChartSize {
        if (containerEl) {
            const rect = containerEl.getBoundingClientRect();
            const innerWidth = rect.width - margin.left - margin.right;
            const innerHeight = rect.height - margin.top - margin.bottom;



            return {
                innerWidth,
                innerHeight,
                outerWidth: rect.width,
                outerHeight: rect.height,
            };
        } else {
            return DEFAULT_SIZE;
        }
    }

    static styles = css`
        :host {
            display: block;
            width: 100%;
            height: 100%;
        }
       
        .bar {
            filter: brightness(1);
            opacity: 1;
            transition: opacity 0.5s ease, transform 0.5s ease;
            pointer-events: none;
        }

        text { 
            pointer-events: none;
        }
        
        .label {
            pointer-events: none;
            opacity: 1;
            transition: opacity 1s, display 1s allow-discrete, overlay 1s allow-discrete;
        }
        .bar:hover {
            /* filter: brightness(1.2); */
            /* opacity: 0.9; */
        }
        .bar.clicked {

        }
        .label.clicked {
            opacity: 1;
            filter: brightness(1.5);
        }
        ::slotted([slot="panel"]) {
            display: none;
            opacity: 0;
            transition: opacity 1s;
        }
        ::slotted([slot="panel"][selected]) {
            display: block;
            opacity: 1;
            
        }

        .container {
            width: 100%;
            height: 100%;
            display: flex;
            flex-direction: column;
            align-items: center;
            justify-content: center;
            padding: 1rem; /* Adjust this based on your actual requirement, 1rem = 16px */
            background-color: #d4e4ee;
            border-radius: 0.5rem; /* This value approximates 'rounded-lg' */
            gap: 20px;
       
        }

        .filter_container {
            width: 100%;
            height: min-content;
            display: flex;
            flex-direction: row;
            align-items: flex-start;
            justify-content: center;
            gap: 4; 
        }

        .graph_container {
            flex: 1;
            width: 100%;
            min-height: 150px;
            position: relative;
            display: flex;
            flex-direction: column;
            align-items: center;
            justify-content: center;
        }
        
        
    `;

    render() {
        const { yScale, xScale } = this.getScales();

        const sortedData = this.bardata.sort((a, b) => a.start > b.start ? -1 : a.start < b.start ? 1 :  0)

        return html`
        <nav data-full-bleed="true" class="${classMap({container: true})}">
           
            <div class="${classMap({graph_container: true})}" ${ref(this.containerRef)} style="aspect-ratio:${this.aspect}">
                <svg id="interactive-svg" class="rounded-lg shadow" data-interactive="true" xmlns="http://www.w3.org/2000/svg" width="100%" height="100%" viewBox="0 0 ${this.size.outerWidth} ${this.size.outerHeight}">
                    <defs>
                        <filter id="glow">
                            <feGaussianBlur stdDeviation="8" result="coloredBlur"></feGaussianBlur>
                            <feMerge>
                                <feMergeNode in="coloredBlur"></feMergeNode>
                                <feMergeNode in="SourceGraphic"></feMergeNode>
                            </feMerge>
                        </filter>
                        <pattern id="error-margin" patternUnits="userSpaceOnUse" width="8" height="8"><path d="M 0,8 l 8,-8 M -2,2 l 4,-4 M 6,10 l 4,-4" stroke-width="2" shape-rendering="auto" stroke="#343434" stroke-linecap="square"></path></pattern>
                        ${map(this.icons, icon => {

                            const svgIcon = icon.querySelector('svg');

                            if(!svgIcon) return nothing;

                            const viewBox = svgIcon.getAttribute('viewBox');
                       


                            return svg`<pattern id="${icon.id}-icon" width="1" height="1" viewBox="${viewBox}" patternUnits="objectBoundingBox">${unsafeHTML(icon.querySelector('svg')?.innerHTML)}</pattern>`
                        })}
                    </defs>
                    <g class="zoomable" cursor="grab">
                        <rect width="${this.size.outerWidth}" height="${this.size.outerHeight}" fill="transparent" @click="${this.onBackgroundClick}"></rect>
                        <g ${ref(this.canvasRef)} class="barchart" transform="translate(${this.margin.left}, ${this.margin.top})">
                            ${drawBar(sortedData, this.size.innerWidth, this.size.innerHeight, this.margin,yScale, xScale, this.hoveredBars, this.clickedBars, this.onBarHoverStart.bind(this), this.onBarHoverEnd.bind(this), this.onBarClick.bind(this))}
                            ${drawAxis(sortedData, this.size.innerWidth, this.size.innerHeight, yScale, xScale)}
                            ${drawVerticalLine(this.clickedValue, this.size.innerWidth, this.size.innerHeight, xScale)}
                        </g>
                    </g>
                </svg>
            </div>
            <slot name="panel"></slot>
        </nav>
        `;
    }

    private onResize(): void {
        const containerEl = this.containerRef.value;
        if (containerEl) {
            const size = this.calculateSize(containerEl, this.margin);
            this.size = size;
            this.requestUpdate();
        }
    }
}

customElements.define('bar-chart', BarChart);

export { BarChart };


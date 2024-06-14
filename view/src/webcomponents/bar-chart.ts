import { ChartSize, Margin } from '@/components/charts/types';
import { axisBottom, axisLeft } from 'd3-axis';
import { ScaleBand, ScaleLinear, scaleBand, scaleLinear, scaleOrdinal } from 'd3-scale';
import { select } from 'd3-selection';
import { LitElement, SVGTemplateResult, css, html, svg } from 'lit';
import { classMap } from 'lit-html/directives/class-map.js';
import { createRef, ref } from 'lit-html/directives/ref.js';
import { property } from 'lit/decorators.js';
import { Observable, debounceTime, fromEvent } from 'rxjs';

const DEFAULT_ASPECT_RATIO = 1;
const DEFAULT_HEIGHT = 540;
const DEFAULT_SIZE = { innerHeight: DEFAULT_HEIGHT, innerWidth: DEFAULT_HEIGHT * DEFAULT_ASPECT_RATIO, outerHeight: DEFAULT_HEIGHT, outerWidth: DEFAULT_HEIGHT * DEFAULT_ASPECT_RATIO };

const blues = [
    '#005178',
    '#1178a0',
    '#184ca1',
    '#4e69b1',
    '#5b97ca'
];

const drawBar = (
    data: Array<BarChartDatum>,
    innerWidth: number,
    innerHeight: number,
    hoveredRegions: Set<string>,
    clickedRegions: Set<string>,
    onHoverStart: (id: string) => void,
    onHoverEnd: (id: string) => void,
    onClick: (id: string) => void
): SVGTemplateResult => {

    const xScale = scaleBand()
        .domain(data.map(d => d.id))
        .range([0, innerWidth])
        .padding(0.1);

    const yScale = scaleLinear()
        .domain([0, 100])
        .nice()
        .range([innerHeight, 0]);

    const colorScale = scaleOrdinal().domain(['0','1','2','3','4','5']).range(blues);

   

    return svg`
        <g>
            ${data.map(d => {
                const isHovered = hoveredRegions.has(d.id);
                const isClicked = clickedRegions.has(d.id);
                return svg`
                    <rect 
                        x="${xScale(d.id)}" 
                        y="${yScale(d.value)}" 
                        width="${xScale.bandwidth()}" 
                        height="${innerHeight - yScale(d.value)}"
                        fill="${colorScale(`${data.indexOf(d) % 5}`)}" 
                        class="${classMap({ region: true, hovered: isHovered, clicked: isClicked, bar: true })}" 
                        @pointerover="${() => onHoverStart(d.id)}" 
                        @pointerout="${() => onHoverEnd(d.id)}" 
                        @click="${() => onClick(d.id)}"
                        data-interactive="true"
                    ></rect>
                    <text 
                        x="${xScale(d?.id)! + xScale.bandwidth() / 2}" 
                        y="${yScale(d.value) - 5}" 
                        text-anchor="middle" 
                        font-size="10px" 
                        class="${classMap({ hovered: isHovered, clicked: isClicked, label: true })}"
                    >
                        ${d.label} (${d.value}%)
                    </text>
                `;
            })}
        </g>
    `;
};

export interface BarChartDatum {
    id: string,
    label: string;
    value: number;
}

class BarChart extends LitElement {
    @property({ type: String }) bardata = "[]";
    @property({ type: Number, attribute: 'aspect-ratio' }) aspectRatio = DEFAULT_ASPECT_RATIO;
    @property({ type: Number }) imageHeight = DEFAULT_HEIGHT;
    @property({ type: Set }) hoveredRegions: Set<string> = new Set();
    @property({ type: Set }) clickedRegions: Set<string> = new Set();

    private containerRef = createRef<HTMLDivElement>();
    private canvasRef = createRef<SVGAElement>();
    private margin: Margin;
    private scaleX: ScaleBand<string>;
    private scaleY: ScaleLinear<number, number, never>;
    private resizeObservable$: Observable<Event>;
    private size: ChartSize = DEFAULT_SIZE;

    get imageWidth() {
        return this.aspectRatio * this.imageHeight;
    }
    get data() {
        return JSON.parse(this.bardata);
    }

    onRegionHoverStart(regionId: string) {
        this.hoveredRegions.add(regionId)
        this.requestUpdate();
    }
    onRegionHoverEnd(regionId: string) {
        this.hoveredRegions.delete(regionId)
        this.requestUpdate();
    }
    onRegionClick(regionId: string) {
        this.clickedRegions = new Set([regionId])
        this.requestUpdate();
    }

    constructor() {
        super();
        this.margin = { left: 40, right: 10, top: 20, bottom: 40 };
        this.scaleX = scaleBand().padding(0.1);
        this.scaleY = scaleLinear().domain([0, 100]).nice();

        this.resizeObservable$ = fromEvent(window, "resize").pipe(
            debounceTime(200),
        );
        this.resizeObservable$.subscribe(async () => {
            this.onResize();
            this.requestUpdate();
        });
    }

    updated(changedProperties: Map<string, any>) {
        if (changedProperties.has('aspectRatio') || changedProperties.has('imageHeight') || changedProperties.has('bardata')) {
            this.requestUpdate();
        }
    }

    firstUpdated() {
        const containerEl = this.containerRef.value!;
        const size = this.calculateSize(containerEl, this.margin);
        this.scaleX.range([0, size.innerWidth]);
        this.scaleY.range([size.innerHeight, 0]);
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

    static styles = [
        css`
            :host {
                display: block;
            }
            .container {
                width: 100%;
                height: 100%;
                min-height: 40vh;
                max-height: 80vh;
                position: relative;
                display: flex;
                align-items: center;
                justify-content: center;
                border-radius: 20px;
                overflow: hidden;
            }
            .path {
                filter: brightness(1);
                opacity: 0.8;
                transition: opacity 0.5s ease, transform 0.5s ease;
            }
            .label {
                pointer-events: none;
                opacity: 1;
                transition: opacity 1s, display 1s allow-discrete, overlay 1s allow-discrete;
            }
           
            .path.hovered {
                filter: brightness(1.2);
                opacity: 0.9;
            }
            .path.clicked, .label.clicked {
                opacity: 1;
                filter: brightness(1.5);
            }
            .centre_label {
                pointer-events: none;
                opacity: 0;
                display: hidden;
                transition: opacity 0.1s, display 1s allow-discrete, overlay 1s allow-discrete;
            }
           
            .centre_label.clicked {
                opacity: 1;
                display: block;
            }
            .x-axis path,
            .y-axis path {
                display: none;
            }
            .x-axis line,
            .y-axis line {
                stroke: #ddd;
            }
        `
    ];

    render() {

        const xAxis = axisBottom(this.scaleX);
        const yAxis = axisLeft(this.scaleY).ticks(10).tickFormat(d => `${d}%`);

        if(this.canvasRef.value){
            const gx = select(this.canvasRef.value).append("g").call(xAxis);
        }

        return html`
            <div ${ref(this.containerRef)} class=${classMap({ container: true })}>
                <svg id="interactive-svg" class="rounded-lg shadow" preserveAspectRatio="xMidYMid meet" data-interactive="true" xmlns="http://www.w3.org/2000/svg" width="100%" height="100%" viewBox="0 0 ${this.size.innerWidth} ${this.size.innerHeight}">
                    <defs>
                        <filter id="glow">
                            <feGaussianBlur stdDeviation="8" result="coloredBlur"></feGaussianBlur>
                            <feMerge>
                                <feMergeNode in="coloredBlur"></feMergeNode>
                                <feMergeNode in="SourceGraphic"></feMergeNode>
                            </feMerge>
                        </filter>
                    </defs>
                    <g class="zoomable" cursor="grab">
                        <g ${ref(this.canvasRef)} class="barchart" transform="translate(${this.margin.left}, ${this.margin.top})">
                            ${drawBar(this.data, this.size.innerWidth, this.size.innerHeight, this.hoveredRegions, this.clickedRegions, this.onRegionHoverStart.bind(this), this.onRegionHoverEnd.bind(this), this.onRegionClick.bind(this))}
                        </g>
                    </g>
                </svg>
            </div>
        `;
    }

    private onResize(): void {
        const containerEl = this.containerRef.value;
        if (containerEl) {
            const size = this.calculateSize(containerEl, this.margin);
            this.size = size;
            this.scaleX.range([0, size.innerWidth]);
            this.scaleY.range([size.innerHeight, 0]);
        }
    }
}

customElements.define('bar-chart', BarChart);

export { BarChart };


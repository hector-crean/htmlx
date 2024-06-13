import { ChartSize, Margin } from '@/components/charts/types';
import { ScaleLinear, scaleLinear, scaleOrdinal } from 'd3-scale';
import { PieArcDatum, arc, pie } from 'd3-shape';
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

]

const drawPie = (
    data: Array<PieChartDatum>,
    innerWidth: number,
    innerHeight: number,
    hoveredRegions: Set<string>,
    clickedRegions: Set<string>,
    onHoverStart: (id: string) => void,
    onHoverEnd: (id: string) => void,
    onClick: (id: string) => void
): SVGTemplateResult => {

    const pieFn = pie<PieChartDatum>()
        .padAngle(0)
        .value(d => d.value);

    const pieData = pieFn(data);

    const radius = 0.5 * Math.min(innerWidth, innerHeight);

    const arcFn = arc<PieArcDatum<PieChartDatum>>()
        .innerRadius(radius * 0.3)
        .outerRadius(radius); // Adjust as needed

        const colorScale = scaleOrdinal().domain(['0','1','2','3','4','5']).range(blues)

    return svg`
        ${pieData.map(d => {
        const isHovered = hoveredRegions.has(d.data.id);
        const isClicked = clickedRegions.has(d.data.id);
        return svg`
                <path 
                    d="${arcFn(d)!}" 
                    fill="${colorScale(`${d.index % 5}`)}" 
                    class="${classMap({ region: true, hovered: isHovered, clicked: isClicked, path: true })}" 
                    @pointerover="${() => onHoverStart(d.data.id)}" 
                    @pointerout="${() => onHoverEnd(d.data.id)}" 
                    @click="${() => onClick(d.data.id)}"
                    data-interactive="true"
                ></path>
                <g
                transform="translate(${arcFn.centroid(d)})"
                >
                    <text 
                        class="${classMap({ hovered: isHovered, clicked: isClicked, label: true })}" 
                        text-anchor="middle" 
                        font-size="10px" 
                        stroke="white"
                    >
                        <tspan x="0" dy="1.2em">${d.data.label}</tspan>
                        <tspan x="0" dy="1.2em">${d.data.value}%</tspan>
                    </text>
                </g>
                <g transform="translate(0, -1.2)">
                    <text 
                        class="${classMap({ hovered: isHovered, clicked: isClicked, centre_label: true })}" 
                        text-anchor="middle" 
                        font-size="25" 
                        stroke="white"
                        
                    >
                        <tspan x="0" dy="1.2em">${d.data.value}%</tspan>
                    </text>
                </g>
            `;
    })}
    `;
}
export interface PieChartDatum {
    id: string,
    label: string;
    value: number;
}

class PieChart extends LitElement {
    @property({ type: String, }) piedata = "[]";
    @property({ type: Number, attribute: 'aspect-ratio' }) aspectRatio = DEFAULT_ASPECT_RATIO;
    @property({ type: Number }) imageHeight = DEFAULT_HEIGHT;
    @property({ type: Set }) hoveredRegions: Set<string> = new Set();
    @property({ type: Set }) clickedRegions: Set<string> = new Set();

    private containerRef = createRef<HTMLDivElement>();
    private margin: Margin;
    private scaleX: ScaleLinear<number, number, never>;
    private scaleY: ScaleLinear<number, number, never>;
    private resizeObservable$: Observable<Event>;
    private size: ChartSize = DEFAULT_SIZE;

    get imageWidth() {
        return this.aspectRatio * this.imageHeight;
    }
    get data() {
        return JSON.parse(this.piedata);
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
        // if (this.clickedRegions.has(regionId)) {
        //     this.clickedRegions.delete(regionId)
        // } else {
        //     this.clickedRegions.add(regionId)
        // }
        this.clickedRegions = new Set([regionId])
        this.requestUpdate();
    }

    constructor() {
        super();
        this.margin = { left: 0, right: 0, top: 0, bottom: 0 };
        this.scaleX = scaleLinear().domain([0, this.imageWidth]);
        this.scaleY = scaleLinear().domain([0, this.imageHeight]);


        this.resizeObservable$ = fromEvent(window, "resize").pipe(
            debounceTime(200),
        );
        this.resizeObservable$.subscribe(async () => {
            this.onResize();
            this.requestUpdate();
        });
    }

    updated(changedProperties: Map<string, any>) {
        if (changedProperties.has('aspectRatio') || changedProperties.has('imageHeight') || changedProperties.has('piedata')) {
            this.requestUpdate();
        }
    }

    firstUpdated() {
        const containerEl = this.containerRef.value!;
        const size = this.calculateSize(containerEl, this.margin);
        this.scaleX = this.scaleX.range([0, size.innerWidth]);
        this.scaleY = this.scaleY.range([0, size.innerHeight]);
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

        `
    ];

    render() {
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
                        <g class="pie" transform="translate(${this.size.innerWidth / 2}, ${this.size.innerHeight / 2})"}>
                            ${drawPie(this.data, this.size.innerWidth, this.size.innerHeight, this.hoveredRegions, this.clickedRegions, this.onRegionHoverStart.bind(this), this.onRegionHoverEnd.bind(this), this.onRegionClick.bind(this))}
                        </g>
                        <g>
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
            this.scaleY.range([0, size.innerHeight]);
        }
    }
}

customElements.define('pie-chart', PieChart);

export { PieChart };


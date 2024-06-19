import { ChartSize, Margin } from "@/lib/types/chart";
import { ScaleBand, ScaleLinear, scaleBand, scaleLinear, scaleOrdinal } from 'd3-scale';
import { LitElement, SVGTemplateResult, css, html, svg } from 'lit';
import { classMap } from 'lit-html/directives/class-map.js';
import { createRef, ref } from 'lit-html/directives/ref.js';
import { property } from 'lit/decorators.js';
import { Observable, debounceTime, filter, fromEvent } from 'rxjs';

const DEFAULT_ASPECT_RATIO = 1;
const DEFAULT_HEIGHT = 540;
const DEFAULT_SIZE = { innerHeight: DEFAULT_HEIGHT, innerWidth: DEFAULT_HEIGHT * DEFAULT_ASPECT_RATIO, outerHeight: DEFAULT_HEIGHT, outerWidth: DEFAULT_HEIGHT * DEFAULT_ASPECT_RATIO };

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

const blues = ['#005178', '#1178a0', '#184ca1', '#4e69b1', '#5b97ca'];

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
        return lines.map((line, index) => svg`
            <tspan x="-10" dy="${index === 0 ? 0 : 1.2}em">${line}</tspan>
        `);
    };

    return svg`
        <g>
            <g transform="translate(0, ${innerHeight})" class="x-axis">
                ${xScale.ticks().map(tick => svg`
                    <g transform="translate(${xScale(tick)}, 0)">
                        <line y2="-${innerHeight}" stroke="#A1C6DD"></line>
                        <text y="20" text-anchor="middle" font-size="10px" class="tick">${tick}</text>
                    </g>
                `)}
            </g>
            <g class="y-axis">
                ${yScale.domain().map(tick => svg`
                    <text x="-10" y="${yScale(tick)! + yScale.bandwidth() / 2}" text-anchor="end" dy="0.32em" font-size="10px" class="tick">
                        ${drawTickLabel(tick)}
                    </text>
                `)}
            </g>
        </g>
    `;
};

const drawBar = (
    data: Array<BarChartDatum>,
    innerWidth: number,
    innerHeight: number,
    yScale: ScaleBand<string>,
    xScale: ScaleLinear<number, number, never>,
    hoveredRegions: Set<string>,
    clickedRegions: Set<string>,
    onHoverStart: (id: string) => void,
    onHoverEnd: (id: string) => void,
    onClick: (id: string) => void
): SVGTemplateResult => {

    const colorScale = scaleOrdinal().domain([...Array(data.length).keys()].map(n => `${n}`)).range(blues);

    return svg`
        <g>
            ${data.map(d => {
        const isHovered = hoveredRegions.has(d.id);
        const isClicked = clickedRegions.has(d.id);
        return svg`
                    <rect 
                        y="${yScale(d.id)}" 
                        x="0"
                        height="${yScale.bandwidth()}" 
                        width="${xScale(d.value)}"
                        fill="${d.fill ? d.fill : colorScale(`${data.indexOf(d) % 5}`)}" 
                        class="${classMap({ bar: true, hovered: isHovered, clicked: isClicked })}" 
                        @pointerover="${() => onHoverStart(d.id)}" 
                        @pointerout="${() => onHoverEnd(d.id)}" 
                        @click="${() => onClick(d.id)}"
                        data-interactive="true"
                    ></rect>
                    <text 
                        x="${xScale(d.value) + 5}" 
                        y="${yScale(d.id)! + yScale.bandwidth() / 2}" 
                        dy="0.32em"
                        text-anchor="start" 
                        font-size="10px" 
                        class="${classMap({ hovered: isHovered, clicked: isClicked, label: true })}"
                    >
                        ${d.value}%
                    </text>
                `;
    })}
        </g>
    `;
};

export interface BarChartDatum {
    id: string,
    fill?: string,
    label: string;
    value: number;
}

class BarChart extends LitElement {
    @property({ type: Array }) bardata: Array<BarChartDatum> = []
    @property({ type: Number, attribute: 'aspect-ratio' }) aspectRatio = DEFAULT_ASPECT_RATIO;
    @property({ type: Number }) imageHeight = DEFAULT_HEIGHT;
    @property({ type: Set }) hoveredRegions: Set<string> = new Set();
    @property({ type: Set }) clickedRegions: Set<string> = new Set();
    @property({ type: Number }) clickedValue: number | null = null;

    private containerRef = createRef<HTMLDivElement>();
    private canvasRef = createRef<SVGAElement>();
    private margin: Margin;
    private resizeObservable$: Observable<Event>;
    private clickOutside$: Observable<Event>;
    private size: ChartSize = DEFAULT_SIZE;

    constructor() {
        super();
        this.margin = { left: 80, right: 0, top: 40, bottom: 20 };

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
        const clickedDatum = this.bardata.find(d => d.id === regionId);
        this.clickedValue = clickedDatum ? clickedDatum.value : null;
        this.clickedRegions = new Set([regionId]);
        this.requestUpdate();
    }

    onBackgroundClick(){
        this.clickedRegions = new Set([]);
        this.clickedValue = null;
        this.requestUpdate();
    }

    private getScales() {
        const ys = this.bardata.map(d => d.id);
        const yScale = scaleBand<string>()
            .domain(ys)
            .range([0, this.size.innerHeight])
            .padding(0.1);

        const xs = this.bardata.map(d => d.value)
        const xScale = scaleLinear()
            .domain([Math.min(0, ...xs), Math.max(...xs)])
            .nice()
            .range([0, this.size.innerWidth]);

        return { yScale, xScale };
    }

    updated(changedProperties: Map<string, any>) {
        if (changedProperties.has('aspectRatio') || changedProperties.has('imageHeight') || changedProperties.has('bardata')) {
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
        .bar {
            filter: brightness(1);
            opacity: 0.8;
            transition: opacity 0.5s ease, transform 0.5s ease;
        }
        .label {
            pointer-events: none;
            opacity: 1;
            transition: opacity 1s, display 1s allow-discrete, overlay 1s allow-discrete;
        }
        .bar.hovered {
            filter: brightness(1.2);
            opacity: 0.9;
        }
        .bar.clicked, .label.clicked {
            opacity: 1;
            filter: brightness(1.5);
        }
        
    `;

    render() {
        const { yScale, xScale } = this.getScales();

        return html`
            <div ${ref(this.containerRef)} class=${classMap({ container: true })}>
                <svg id="interactive-svg" class="rounded-lg shadow" data-interactive="true" xmlns="http://www.w3.org/2000/svg" width="100%" height="100%" viewBox="0 0 ${this.size.outerWidth} ${this.size.outerHeight}">
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
                        <rect width="${this.size.outerWidth}" height="${this.size.outerHeight}" fill="transparent" @click="${this.onBackgroundClick}"></rect>
                        <g ${ref(this.canvasRef)} class="barchart" transform="translate(${this.margin.left}, ${this.margin.top})">
                            ${drawAxis(this.bardata, this.size.innerWidth, this.size.innerHeight, yScale, xScale)}
                            ${drawBar(this.bardata, this.size.innerWidth, this.size.innerHeight, yScale, xScale, this.hoveredRegions, this.clickedRegions, this.onRegionHoverStart.bind(this), this.onRegionHoverEnd.bind(this), this.onRegionClick.bind(this))}
                            ${drawVerticalLine(this.clickedValue, this.size.innerWidth, this.size.innerHeight, xScale)}
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
            this.requestUpdate();
        }
    }
}

customElements.define('bar-chart', BarChart);

export { BarChart };


import { ChartSize, Margin } from '@/webcomponents/chart.types';
import { hierarchy, partition } from 'd3-hierarchy';
import { ScaleOrdinal, scaleOrdinal } from 'd3-scale';
import { select } from 'd3-selection';
import { ZoomBehavior, zoom } from 'd3-zoom';
import { LitElement, SVGTemplateResult, css, html, svg } from 'lit';
import { classMap } from 'lit-html/directives/class-map.js';
import { createRef, ref } from 'lit-html/directives/ref.js';
import { property } from 'lit/decorators.js';
import { Observable, debounceTime, fromEvent } from 'rxjs';

const DEFAULT_ASPECT_RATIO = 1;
const DEFAULT_HEIGHT = 540;
const DEFAULT_SIZE = { innerHeight: DEFAULT_HEIGHT, innerWidth: DEFAULT_HEIGHT * DEFAULT_ASPECT_RATIO, outerHeight: DEFAULT_HEIGHT, outerWidth: DEFAULT_HEIGHT * DEFAULT_ASPECT_RATIO };

const colors = [
    '#005178',
    '#1178a0',
    '#184ca1',
    '#4e69b1',
    '#5b97ca'
];

type UUID = string;

interface Branch {
    type: "Branch";
    id: UUID;
    label: string;
    fill?: string;
    children: PartitionNode[];
}

interface Leaf {
    type: "Leaf";
    id: UUID;
    label: string;
    fill?: string;
    value: number;
}

type PartitionNode = Branch | Leaf;

const drawPartition = (
    data: Array<PartitionNode>,
    size: ChartSize,
    colorScale: ScaleOrdinal<string, string>,
    hoveredRegions: Set<String>,
    clickedRegions: Set<String>,
    onHoverStart: (id: string) => void,
    onHoverEnd: (id: string) => void,
    onClick: (id: string) => void
): SVGTemplateResult => {

    const partitionFn = partition<PartitionNode>()
        .size([size.innerWidth, size.innerHeight]);

    const root = hierarchy({ children: data } as any)
        .sum(d => d.value)
        .sort((a, b) => b.value! - a.value!);

    const hierarchyPartitionNode = partitionFn(root);

    const descendants = hierarchyPartitionNode.descendants();

    return svg`
        <g>
            ${descendants.map(node => {
        const isLeaf = !node.children;
        const isHovered = hoveredRegions.has(node.data.id);
        const isClicked = clickedRegions.has(node.data.id);
        return svg`
                    <g transform="translate(${node.x0}, ${node.y0})" 
                       @pointerover="${() => onHoverStart(node.data.id)}" 
                       @pointerout="${() => onHoverEnd(node.data.id)}" 
                       @click="${() => onClick(node.data.id)}">
                        <rect 
                            width="${node.x1 - node.x0}" 
                            height="${node.y1 - node.y0}" 
                            fill="${isLeaf ? (node.data.fill ? node.data.fill : colorScale(node.data.id)) : 'none'}" 
                            stroke="${isLeaf ? 'none' : colorScale(node.data.id)}"
                            class="${classMap({ region: true, hovered: isHovered, clicked: isClicked })}"></rect>
                        <text 
                            x="${(node.x1 - node.x0) / 2}" 
                            y="${(node.y1 - node.y0) / 2}" 
                            text-anchor="middle" 
                            font-size="12px" 
                            class="${classMap({ label: true, hovered: isHovered, clicked: isClicked })}">
                            ${node.data.label}
                        </text>
                    </g>
                `;
    })}
        </g>
    `;
};

class PartitionChart extends LitElement {
    @property({ type: String }) partitiondata = "[]";
    @property({ type: Number, attribute: 'aspect-ratio' }) aspectRatio = DEFAULT_ASPECT_RATIO;
    @property({ type: Number }) imageHeight = DEFAULT_HEIGHT;
    @property({ type: Set }) hoveredRegions: Set<string> = new Set();
    @property({ type: Set }) clickedRegions: Set<string> = new Set();

    private containerRef = createRef<HTMLDivElement>();
    private canvasRef = createRef<SVGAElement>();
    private margin: Margin;
    private resizeObservable$: Observable<Event>;
    private size: ChartSize = DEFAULT_SIZE;

    get imageWidth() {
        return this.aspectRatio * this.imageHeight;
    }
    get data(): Array<PartitionNode> {
        return JSON.parse(this.partitiondata);
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
        this.margin = { left: 30, right: 0, top: 40, bottom: 20 };

        this.resizeObservable$ = fromEvent(window, "resize").pipe(
            debounceTime(200),
        );
        this.resizeObservable$.subscribe(async () => {
            this.onResize();
            this.requestUpdate();
        });
    }

    updated(changedProperties: Map<string, any>) {
        if (changedProperties.has('aspectRatio') || changedProperties.has('imageHeight') || changedProperties.has('partitiondata')) {
            this.requestUpdate();
        }
    }

    firstUpdated() {
        const containerEl = this.containerRef.value!;
        const size = this.calculateSize(containerEl, this.margin);
        this.onResize();

        const svg = select(this.shadowRoot?.querySelector("svg")!);
        const g = select(this.shadowRoot?.querySelector(".zoomable")!);

        const zoomBehavior: ZoomBehavior<SVGSVGElement, unknown> = zoom<SVGSVGElement, unknown>()
            .scaleExtent([0.5, 5])
            .on('zoom', (event) => {
                g.attr('transform', event.transform);
            });

        svg.call(zoomBehavior);
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
            .region {
                opacity: 0.8;
                transition: opacity 0.5s ease, transform 0.5s ease;
            }
            .label {
                pointer-events: none;
                opacity: 1;
                transition: opacity 1s, display 1s allow-discrete, overlay 1s allow-discrete;
            }
            .region.hovered {
                opacity: 0.9;
            }
            .region.clicked, .label.clicked {
                opacity: 1;
                filter: brightness(1.5);
            }
        `
    ];

    render() {
        const colorScale = scaleOrdinal<string>().domain(this.data.map(d => d.id)).range(colors);

        return html`
        <div ${ref(this.containerRef)} class=${classMap({ container: true })}>
        <svg id="interactive-svg" class="rounded-lg shadow" data-interactive="true" xmlns="http://www.w3.org/2000/svg" width="100%" height="100%" viewBox="0 0 ${this.size.outerWidth} ${this.size.outerHeight}">
            <g class="zoomable" cursor="grab">
                <g ${ref(this.canvasRef)} class="partitionchart" transform="translate(${this.margin.left}, ${this.margin.top})">
                    ${drawPartition(this.data, this.size, colorScale, this.hoveredRegions, this.clickedRegions, this.onRegionHoverStart.bind(this), this.onRegionHoverEnd.bind(this), this.onRegionClick.bind(this))}
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
        }
    }
}

customElements.define('partition-chart', PartitionChart);

export { PartitionChart };


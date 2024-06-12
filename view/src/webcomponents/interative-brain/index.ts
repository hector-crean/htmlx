import { ChartSize, Margin } from "@/components/charts/types";
import { ScaleLinear, scaleLinear } from "d3-scale";
import { curveNatural, line } from "d3-shape";
import { LitElement, SVGTemplateResult, css, html, svg } from "lit";
import { classMap } from "lit-html/directives/class-map.js";
import { createRef, ref } from "lit-html/directives/ref.js";
import { property } from "lit/decorators.js";
import { map } from "lit/directives/map.js";
import { Observable, debounceTime, fromEvent } from "rxjs";
import defaultBackground from "./brain-background.jpg";
import { pathways } from "./brain-pathways";
import { regions } from "./brain-regions";

class BrainRegion extends LitElement {
	@property({ type: Boolean })
	visible = false;

	protected render() {
		return svg`
    
    `;
	}
}

const drawRegion = (
	{ points, fillColor, id: regionId }: Region,
	scaleX: ScaleLinear<number, number, never>,
	scaleY: ScaleLinear<number, number, never>,
	hovered: boolean,
	clicked: boolean,
	onHoverStart: (id: string) => void,
	onHoverEnd: (id: string) => void,
	onClick: (id: string) => void,
): SVGTemplateResult => {
	const pointsStr = points
		.map(({ x, y }) => `${scaleX(x)},${scaleY(y)}`)
		.join(" ");

	return svg`<polygon id="${regionId}" popovertarget="${regionId}-popover" @pointerover="${() =>
		onHoverStart(regionId)}"  @pointerout="${() =>
		onHoverEnd(regionId)}" @click="${() =>
		onClick(regionId)}" class="${classMap({
		region: true,
		hovered,
		clicked,
		polygon: true,
	})}" data-interactive="true" points="${pointsStr}" fill="${fillColor}" filter="url(#glow)" opacity="0.45"></polygon>`;
};

const drawPathway = (
	{ points }: Pathway,
	scaleX: ScaleLinear<number, number, never>,
	scaleY: ScaleLinear<number, number, never>,
): SVGTemplateResult => {
	const curve = line().curve(curveNatural);
	const project = (p: Point): [number, number] => [scaleX(p.x), scaleY(p.y)];
	const pointsArr = points.map(project);
	const d = points ? curve(pointsArr as [number, number][]) : "";

	return svg`<path d="${d}" fill="none" stroke="black"></path>`;
};

const drawCentroid = (
	{ centroid: { x, y } }: Region,
	scaleX: ScaleLinear<number, number, never>,
	scaleY: ScaleLinear<number, number, never>,
): SVGTemplateResult => {
	return svg`<circle data-interactive="true" cx="${scaleX(x)}" cy="${scaleY(
		y,
	)}" r="5" fill="white"></circle>`;
};

const drawLabel = (
	region: Region,
	scaleX: ScaleLinear<number, number, never>,
	scaleY: ScaleLinear<number, number, never>,
	hovered: boolean,
	clicked: boolean,
): SVGTemplateResult => {
	const project = (p: Point): [number, number] => [scaleX(p.x), scaleY(p.y)];
	const add = (p1: Point, p2: Point) => ({ x: p1.x + p2.x, y: p1.y + p2.y });

	const lineFunction = line<[number, number]>()
		.x((d) => d[0])
		.y((d) => d[1]);

	const sOffsetX = region.label.position.x < region.centroid?.x ? -10 : 10;
	const sOffsetY = region.label.position.y < region.centroid?.y ? -10 : 10;
	const eOffsetX = region.label.position.x < region.centroid?.x ? 85 : -85;
	const eOffsetY = region.label.position.y < region.centroid?.y ? 20 : -20;

	const startPoint = region.label.altDrawMode
		? { x: region.centroid?.x, y: region.centroid?.y + sOffsetY }
		: { x: region.centroid?.x + sOffsetX, y: region.centroid?.y };
	const midPoint = region.label.altDrawMode
		? { x: region.centroid?.x, y: region.label.position.y }
		: { x: region.label.position.x, y: region.centroid?.y };
	const endPoint = !region.label.altDrawMode
		? { x: region.label.position.x, y: region.label.position.y + eOffsetY }
		: { x: region.label.position.x + eOffsetX, y: region.label.position.y };

	const lineData = [startPoint, midPoint, endPoint].map(project);

	const d = lineFunction(lineData);

	return svg`
  <circle class="${classMap({
		
		clicked,
		hovered,
		label: true,
	})}" data-interactive="true" cx="${scaleX(region.centroid.x)}" cy="${scaleY(
		region.centroid.y,
	)}" r="5" fill="white"></circle>
    <path class="${classMap({
			
			clicked,
			hovered,
			label: true,
		})}"  data-interactive="true" d="${d}" stroke-width="2" stroke="white" fill="none" stroke-dasharray="" stroke-dashoffset="0"></path>
    <rect class="${classMap({
			region: true,
			clicked,
			hovered,
			label: true,
		})}"  width="${scaleX(250) - scaleX(0)}" height="${
		scaleY(30) - scaleY(0)
	}" x="${scaleX(region.label.position.x - 125)}" y="${scaleY(
		region.label.position.y - 15,
	)}" opacity="1" fill="white"></rect>
    <text class="${classMap({
			
			clicked,
			hovered,
			label: true,
		})}"  x="${scaleX(region.label.position.x)}" y="${scaleY(
		region.label.position.y) + 5}" text-anchor="middle" opacity="1" font-size="10px" >${region.name}</text>
  `;
};

interface Region {
	id: string;
	name: string;
	points: Point[];
	centroid: Point;
	fillColor: string;
	label: Label;
}

interface Pathway {
	id: string;
	points: Point[];
	color: string;
	width?: number;
	glow?: boolean;
	labels: Label[];
}

interface Point {
	x: number;
	y: number;
}

interface Label {
	position: Point;
	altDrawMode?: boolean;
	text?: string;
	size?: { width: number; height: number };
	offset?: { x: number; y: number };
}

type Regions = Array<Region>;
type Pathways = Array<Pathway>;

const DEFAULT_ASPECT_RATIO = 1280 / 720;
const DEFAULT_HEIGHT = 540;
const DEFAULT_SIZE = {
	innerHeight: DEFAULT_HEIGHT,
	innerWidth: DEFAULT_HEIGHT * DEFAULT_ASPECT_RATIO,
	outerHeight: DEFAULT_HEIGHT,
	outerWidth: DEFAULT_HEIGHT * DEFAULT_ASPECT_RATIO,
};

class InteractiveBrain extends LitElement {
	@property({ type: Number, attribute: "aspect-ratio" }) aspectRatio =
		DEFAULT_ASPECT_RATIO;
	@property({ type: Number }) imageHeight = DEFAULT_HEIGHT;
	@property({ type: String }) background: string = defaultBackground;
	@property({ type: Set }) hoveredRegions: Set<string> = new Set();
	@property({ type: Set }) clickedRegions: Set<string> = new Set();

	// @property({ type: String, attribute: 'regions' }) regionsStr = JSON.stringify(regions);
	// @property({ type: String, attribute: 'pathways' }) pathswaysStr = JSON.stringify(pathways);

	@property({ type: String, attribute: "regions" }) regionIdsStr = "[]";

	private regions: Array<Region>;
	private pathways: Array<Pathway>;
	private containerRef = createRef<HTMLDivElement>();
	private margin: Margin;
	private scaleX: ScaleLinear<number, number, never>;
	private scaleY: ScaleLinear<number, number, never>;
	private resizeObservable$: Observable<Event>;
	private size: ChartSize = DEFAULT_SIZE;

	// get regions(): Regions {
	//   return JSON.parse(this.regionsStr);
	// }
	// get pathways(): Pathways {
	//   return JSON.parse(this.pathswaysStr);
	// }

	get regionIds(): Array<string> {
		const regionsIds = JSON.parse(this.regionIdsStr);
		return regionsIds;
	}

	get imageWidth() {
		return this.aspectRatio * this.imageHeight;
	}

	onRegionHoverStart(regionId: string) {
		this.hoveredRegions.add(regionId);
		this.requestUpdate();
	}
	onRegionHoverEnd(regionId: string) {
		this.hoveredRegions.delete(regionId);
		this.requestUpdate();
	}
	onRegionClick(regionId: string) {
		if (this.clickedRegions.has(regionId)) {
			this.clickedRegions.delete(regionId);
		} else {
			this.clickedRegions.add(regionId);
		}
		this.requestUpdate();
	}

	constructor() {
		super();
		const margin = { left: 0, right: 0, top: 0, bottom: 0 };

		this.regions = regions.filter((region) =>
			this.regionIds.includes(region.id),
		);
		this.pathways = pathways;

		this.margin = margin;
		this.background = defaultBackground;

		this.scaleX = scaleLinear().domain([0, this.imageWidth]);
		this.scaleY = scaleLinear().domain([0, this.imageHeight]);

		this.hoveredRegions = new Set();

		this.resizeObservable$ = fromEvent(window, "resize").pipe(
			debounceTime(200),
		);
		this.resizeObservable$.subscribe(async () => {
			this.onResize();
			this.requestUpdate();
		});
	}

	updated(changedProperties: Map<string, any>) {
    if(changedProperties.has('regionIdsStr')) {
      this.regions = regions.filter((region) =>
				this.regionIds.includes(region.id),
			);
    }
		if (
			changedProperties.has("aspectRatio") ||
			changedProperties.has("imageHeight") ||
			changedProperties.has("regionIdsStr")
		) {
			
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

	private calculateSize(
		containerEl: HTMLElement | undefined,
		margin: Margin,
	): ChartSize {
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
      position: relative;
      display: flex;
      align-items: center;
      justify-content: center;
      border-radius: 20px;
      overflow: hidden;
    }


    .polygon {
      opacity: 0.1;
      transition: opacity 0.5s ease, transform 0.5s ease;
    }
    .label {
      opacity: 0;
      display: hidden;
      transition: opacity 1s, display 1s allow-discrete, overlay 1s allow-discrete;
    }
    .label.hovered {
      opacity: 0.5;
      display: block;
    }
    
    
    .polygon.hovered {
      opacity: 0.5;
    }
    .polygon.clicked, .label.clicked {
      display: block;
      opacity: 0.9;

    }

    
  `,
	];

	render() {
		return html`
      <div ${ref(this.containerRef)} class=${classMap({ container: true })}>      
        <modal-dialog>
        <div slot="title">
        <p>Title</p>
       </div>
          <div slot="content">
           <p> Hover / Click regions of the brain to see descriptions</p>
          </div>
        </modal-dialog>
        <svg id="interactive-svg" class="rounded-lg shadow" preserveAspectRatio="xMidYMid meet" data-interactive="true" xmlns="http://www.w3.org/2000/svg" width="100%" height="100%" viewBox="0 0 ${
					this.size.innerWidth
				} ${this.size.innerHeight}">
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
            <g class="bg-images">
              <image x="0" y="0" width="${this.size.innerWidth}" height="${
			this.size.innerHeight
		}" xlink:href="/brain-background.jpg" />
            </g>
            <g class="regions">
              ${map(this.regions, (region) =>
								drawRegion(
									region,
									this.scaleX,
									this.scaleY,
									this.hoveredRegions.has(region.id),
									this.clickedRegions.has(region.id),
									this.onRegionHoverStart.bind(this),
									this.onRegionHoverEnd.bind(this),
									this.onRegionClick.bind(this),
								),
							)}
            </g>
             <g class="pathways">
            </g>
            
            <g class="labels">
              ${map(this.regions, (region) =>
								drawLabel(
									region,
									this.scaleX,
									this.scaleY,
									this.hoveredRegions.has(region.id),
									this.clickedRegions.has(region.id),
								),
							)}
            </g>
            <g class="info-area"></g>
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

customElements.define("interactive-brain", InteractiveBrain);

export { InteractiveBrain };
export type { Label, Pathway, Pathways, Region, Regions };


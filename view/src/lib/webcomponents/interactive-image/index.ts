import { ChartSize, Margin } from "@/lib/types/chart";
import { ScaleLinear, scaleLinear } from "d3-scale";
import { curveNatural, line } from "d3-shape";
import { LitElement, SVGTemplateResult, css, html, nothing, svg } from "lit";
import { classMap } from "lit-html/directives/class-map.js";
import { createRef, ref } from "lit-html/directives/ref.js";
import { property } from "lit/decorators.js";
import { map } from "lit/directives/map.js";
import { Observable, debounceTime, fromEvent } from "rxjs";
import { brainBgDefs } from "./brain";

const closeIcon = () => svg`
<svg width="15" height="15" viewBox="0 0 15 15" fill="none" xmlns="http://www.w3.org/2000/svg"><path d="M12.8536 2.85355C13.0488 2.65829 13.0488 2.34171 12.8536 2.14645C12.6583 1.95118 12.3417 1.95118 12.1464 2.14645L7.5 6.79289L2.85355 2.14645C2.65829 1.95118 2.34171 1.95118 2.14645 2.14645C1.95118 2.34171 1.95118 2.65829 2.14645 2.85355L6.79289 7.5L2.14645 12.1464C1.95118 12.3417 1.95118 12.6583 2.14645 12.8536C2.34171 13.0488 2.65829 13.0488 2.85355 12.8536L7.5 8.20711L12.1464 12.8536C12.3417 13.0488 12.6583 13.0488 12.8536 12.8536C13.0488 12.6583 13.0488 12.3417 12.8536 12.1464L8.20711 7.5L12.8536 2.85355Z" fill="currentColor" fill-rule="evenodd" clip-rule="evenodd"></path></svg>`


interface PanelElement extends HTMLElement {
	slot: string;
	id: string;
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

	return svg`
	<polygon  
		id="${regionId}" 
		points="${pointsStr}" 
		fill="url(#dense_crosshatch)" 
		opacity="0.45" 
		class="${classMap({
			flashing_glow: clicked,
			region: true,
			hovered,
			clicked,
			polygon: true,
		})}" 
	></polygon>
	<polygon  
		id="${regionId}" 
		popovertarget="${regionId}-popover" 
		@pointerover="${() => onHoverStart(regionId)}" 
		@pointerout="${() => onHoverEnd(regionId)}" 
		@click="${() => onClick(regionId)}" 
		class="${classMap({
			flashing_glow: clicked,
			region: true,
			hovered,
			clicked,
			polygon: true,
		})}" 
		data-interactive="true" 
		points="${pointsStr}" 
		fill="${fillColor}" 
		filter="url(#glow)" 
		opacity="0.45" 
		stroke="black" 
        stroke-width="2" 
        stroke-dasharray="2,3"
	></polygon>
		  
		   `;
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
		})}" data-interactive="true" d="${d}" stroke-width="2" stroke="white" fill="none" stroke-dasharray="" stroke-dashoffset="0" ></path>
    <rect class="${classMap({
			region: true,
			clicked,
			hovered,
			label: true,
		})}"  width="${scaleX(250) - scaleX(0)}" height="${
		scaleY(30) - scaleY(0)
	}" x="${scaleX(region.label.position.x - 125)}" y="${scaleY(
		region.label.position.y - 15,
	)}" opacity="1" fill="white" rx="1em" ry="1em"></rect>
    <text class="${classMap({
			clicked,
			hovered,
			label: true,
		})}"  x="${scaleX(region.label.position.x)}" y="${
		scaleY(region.label.position.y) + 5
	}" text-anchor="middle" opacity="1" font-size="10px" fill="black">${
		region.name
	}</text>
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

class InteractiveImage extends LitElement {
	@property({ type: String }) src = "";
	@property({ type: Number, attribute: "aspect-ratio" }) aspectRatio =
		DEFAULT_ASPECT_RATIO;
	@property({ type: Number }) imageHeight = DEFAULT_HEIGHT;
	@property({ type: Array }) regions: Array<Region> = [];
	@property({ type: Array }) pathways: Array<Pathway> = [];
	@property({ type: Object }) margin: Margin = {
		top: 0,
		left: 0,
		bottom: 0,
		right: 0,
	};
	@property({ type: Boolean }) modalOpen = false;

	private hoveredRegions: Set<string> = new Set();
	private clickedRegions: Set<string> = new Set();
	private panels: Array<PanelElement>;

	private containerRef = createRef<HTMLDivElement>();
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

	handleOpenModal() {
		this.modalOpen = true;
		this.requestUpdate()

	}
	handleCloseModal() {
		this.modalOpen = false;
		console.log(this.modalOpen)
		this.requestUpdate()
	}

	onRegionClick(regionId: string) {
		// if (this.clickedRegions.has(regionId)) {
		// 	this.clickedRegions.delete(regionId);
		// } else {
		// 	this.clickedRegions.add(regionId);
		// }
		this.clickedRegions.clear();
		this.clickedRegions.add(regionId);

		this.panels.forEach((panel) => panel.removeAttribute("selected"));
		this.clickedRegions.forEach((regionId) => {
			const panel = this.panels.find((panel) => panel.id === regionId);
			if (panel) {
				panel.setAttribute("selected", "");
			}
		});
		this.modalOpen = true;

		this.requestUpdate();
	}

	constructor() {
		super();
		this.panels = Array.from(this.querySelectorAll("[slot=panel][id]"));

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

		this.onResize();
	}

	// updated(changedProperties: Map<string, any>) {
	// 	if (changedProperties.has("regionIdsStr")) {
	// 		this.regions = regions.filter((region) =>
	// 			this.regionIds.includes(region.id),
	// 		);
	// 	}
	// 	if (
	// 		changedProperties.has("aspectRatio") ||
	// 		changedProperties.has("imageHeight") ||
	// 		changedProperties.has("regionIdsStr")
	// 	) {
	// 		this.requestUpdate();
	// 	}
	// }

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
	  width: 100%;
	  height: 100%;
	  padding: 0 2px;
    }

	.outer_container {
		position: relative;
      	width: 100%;
		height: 100%;
      	display: flex;
	  	flex-direction: column;
      	align-items: center;
      	justify-content: center;
      	border-radius: 2px;
	}
	@media only screen and (max-width: 650px) {
		.outer_container {
			height: 50dvh;
			background-color: #383f44;
		}
	}
	@media only screen and (max-width: 550px) {
		.outer_container {
			height: 75dvh;
			background-color: #383f44;

		}
	}


    .container {
      	display: flex;
      	align-items: center;
      	justify-content: center;
      	border-radius: 20px;
		width: min(100%, 1000px);
    }


    .polygon {
		cursor: pointer;
    }
    .label {
		pointer-events: none;
      opacity: 0;
      display: hidden;
      transition: opacity 1s, display 1s allow-discrete, overlay 1s allow-discrete;
    }
	.label.hovered {
      opacity: 1;
    }
    

	.region_label {
		border-radius: 20em;

	}
    
    
    .polygon.hovered {
      opacity: 0.5;
    }
    .polygon.clicked, .label.clicked {
      display: block;
      opacity: 0.7;
    }
	.modal {
		position: absolute;
		top: 0;
		left: 50%;
		right: 0;
		bottom: 0;
		background-color: #1e1e1e;
		color: white;
		width: min(100%, 1000px);
		transform: translate(-50%, 0);
		backdrop-filter: blur(10px);
		opacity: 0.8;   
		display: flex;
		flex-direction: column;
		align-items: center;
		justify-content: center;    
		
	}
	.modal_inner {
		width: 100%;
		display: flex;
		flex-direction: column;
		overflow-y: scroll;
		scrollbar-width: none; /* Disables scrollbar in Firefox */

	}
	.modal_inner::-webkit-scrollbar {
  		display: none;	
	}

	.close_modal_btn {
		mix-blend-mode: hard-light;
		background-color: black;
		cursor: pointer;
		color: white;
		position: absolute;
		top: 20px;
		right: 20px;
		z-index: 200;
		display: flex;
		flex-direction: column;
		align-items: center;
		justify-content: center;
		padding: 12px;
		opacity: 0.5;
	}
	.close_modal_btn:hover {
		filter: brightness(1.2);
	}

	::slotted([slot="panel"]) {
        display: none;
        opacity: 0;
        transition: opacity 1s;
    }
    ::slotted([slot="panel"][selected]) {
        display: block;
        opacity: 0.8;    
		padding: 0px 20px;    
    }

	@keyframes flash {
            0%, 100% {
                filter:brightness(0.8);
            }
            50% {
                filter: brightness(1.2);
            }
    }

    /* Apply the animation to the polygon */
    .flashing_glow {
        animation: flash 1s infinite alternate;
    }

    
  `,
	];

	render() {
		return html`
		<div class=${classMap({ outer_container: true })}>
      <div ${ref(this.containerRef)} class=${classMap({
			container: true,
		})} style="aspect-ratio:${this.aspectRatio}">      
        <svg id="interactive-svg" class="rounded-lg shadow" preserveAspectRatio="xMidYMid meet" data-interactive="true" xmlns="http://www.w3.org/2000/svg" width="100%" height="100%" viewBox="0 0 ${
					this.size.innerWidth
				} ${this.size.innerHeight}" fill="none">
        <defs>
            <filter id="glow">
              <feGaussianBlur stdDeviation="8" result="coloredBlur"></feGaussianBlur>
              <feMerge>
                <feMergeNode in="coloredBlur"></feMergeNode>
                <feMergeNode in="SourceGraphic"></feMergeNode>
              </feMerge>
            </filter>
			<pattern id="dense_crosshatch" patternUnits="userSpaceOnUse" width="4" height="4"><path d="M 0,4 l 4,-4 M -1,1 l 2,-2 M 3,5 l 2,-2" stroke-width="1" shape-rendering="auto" stroke="#343434" stroke-linecap="square"></path></pattern>
			<pattern id="dense_circle" patternUnits="userSpaceOnUse" width="5" height="5"><circle cx="2.5" cy="2.5" r="2" fill="#343434" stroke="#343434" stroke-width="0"></circle></pattern>	
		</defs>
		${brainBgDefs()}

          <g class="zoomable" cursor="grab">
            <g class="bg-images">
				<rect xmlns="http://www.w3.org/2000/svg" width="${
					this.size.innerWidth
				}" height="${this.size.innerHeight}" fill="url(#bg_img)"/>
             
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
	  ${
			this.modalOpen
				? html`
		<div class="${classMap({ modal: true })}">
			<div class="${classMap({ modal_inner: true })}" >
				<div class="${classMap({ close_modal_btn: true })}" @click="${
						this.handleCloseModal
				  }">${closeIcon()}</div>
				<slot name="panel"></slot>
			</div>
		</div>
		</div>
	  `
				: nothing
		}
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

customElements.define("interactive-image", InteractiveImage);

export { InteractiveImage };
export type { Label, Pathway, Pathways, Region, Regions };


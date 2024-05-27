import { easeSinInOut } from "d3-ease";
import { scaleLinear } from "d3-scale";
import { pointer, select, selection } from "d3-selection";
import { curveNatural, line } from "d3-shape";
import selection_interrupt from "../../../node_modules/d3-transition/src/selection/interrupt";
import selection_transition from "../../../node_modules/d3-transition/src/selection/transition";
selection.prototype.interrupt = selection_interrupt;
selection.prototype.transition = selection_transition;

import defaultBackground from "./brain-background.jpg";

export class InteractiveBrain {
	constructor(element, params) {
		const defaults = {
			regions: {},
			pathways: {},
			background: defaultBackground,
			interactive: true,
		};

		params = { ...defaults, ...params };

		this.regions = params.regions;
		this.pathways = params.pathways;

		const width = 960;
		const height = 540;

		this.svg = select(element);

		this.svg.attr("data-interactive", true);
		// .attr("viewBox", `0 0 ${width} ${height}`);

		// layers
		this.layer0 = this.svg.append("g").classed("regions", true);
		this.pathwaysGroup = this.svg.append("g").classed("pathways", true);
		this.layer1 = this.svg.append("g").classed("centroids", true);
		this.layer2 = this.svg.append("g").classed("lines", true);
		this.layer3 = this.svg.append("g").classed("labels", true);
		this.layer4 = this.svg.append("g").classed("infoArea", true);

		this.lastSelected = null;

		const scaleX = scaleLinear()
			.domain([0, 960]) //Give appropriate range in the scale
			.range([0, width]);

		const scaleY = scaleLinear()
			.domain([0, 540]) //Give appropriate range in the scale
			.range([0, height]);

		this.svg.on("pointerdown", function (ev) {
			const coordinates = pointer(ev);

			const x = coordinates[0];
			const y = coordinates[1];

			// console.log('{"x":' + x + ', "y":' + y + "},");
		});

		const self = this;
		this.layer0
			.append("svg:image")
			.attr("x", 0)
			.attr("y", 0)
			.attr("width", width)
			.attr("height", height)
			.attr("xlink:href", params.background);

		this.layer0
			.selectAll("polygon")
			.data(Object.keys(this.regions))
			.enter()
			.append("polygon")
			.attr("class", "region")
			.attr("data-interactive", true)
			.attr("points", function (datum, index, groups) {
				self.regions[datum].id = datum;

				if (self.regions[datum].points) {
					self.regions[datum].polygon = this;
					return self.regions[datum].points
						.map(function (datum) {
							return [scaleX(datum.x), scaleY(datum.y)].join(",");
						})
						.join(" ");
				}
			})
			.attr("fill", function (d) {
				return self.regions[d].fillColor;
			})
			.on("pointerdown", function (event, d) {
				if (params.interactive) {
					self.highlightRegions([d]);
				}
			});

		const curve = line().curve(curveNatural);
		this.pathwaysGroup
			.selectAll("path")
			.data(Object.keys(this.pathways))
			.enter()
			.append("path")
			.attr("data-interactive", true)
			.attr("class", "pathway")
			.attr("d", function (datum, index, groups) {
				self.pathways[datum].id = datum;
				self.pathways[datum].path = this;

				const points = self.pathways[datum].points.map((point) => [point.x, point.y]);
				return curve(points);
			})
			.attr("stroke", "none")
			.attr("fill", "none");

		//Container for the gradients
		const defs = this.svg.append("defs");

		//Filter for the outside glow
		const filter = defs.append("filter").attr("id", "glow");
		filter
			.append("feGaussianBlur")
			.attr("stdDeviation", "8")
			.attr("result", "coloredBlur");
		const feMerge = filter.append("feMerge");
		feMerge.append("feMergeNode").attr("in", "coloredBlur");
		feMerge.append("feMergeNode").attr("in", "SourceGraphic");
	}

	clearLabels() {
		// fade out any prior path if it exists
		const priorPath = this.layer2.selectAll("path");
		if (!priorPath.empty()) {
			priorPath.each(function (datum, index, groups) {
				const path = select(this);
				const priorLength = path.node().getTotalLength();
				path
					.attr("stroke-dasharray", `${priorLength} ${priorLength}`)
					.attr("stroke-dashoffset", 0)
					.attr("data-interactive", true)
					.transition()
					.delay(500)
					.duration(500)
					.ease(easeSinInOut)
					.attr("stroke-dashoffset", priorLength)
					.remove();
			});
		}

		const priorLabel = this.layer3.selectAll("rect");
		if (!priorLabel.empty()) {
			priorLabel
				.transition()
				.delay(250)
				.duration(500)
				.attr("opacity", 0)
				.remove();
		}

		const priorText = this.layer3.selectAll("text");
		if (!priorText.empty()) {
			priorText
				.transition()
				.duration(500)
				.attr("opacity", 0)
				.remove();
		}
	}

	clearRegionsAndPathways() {
		this.svg.selectAll("polygon").classed("active", false);

		const pathways = this.pathwaysGroup.selectAll("path");
		if (!pathways.empty()) {
			pathways
				.transition()
				.duration(500)
				.ease(easeSinInOut)
				.attr("stroke-width", 0);
		}
	}

	clearCentroids() {
		const centroids = this.layer1.selectAll("circle");
		if (!centroids.empty()) {
			centroids.remove();
		}
	}

	isAlreadySelected(data) {
		if (this.lastSelected === data.join("-")) {
			return true;
		}
		this.lastSelected = data.join("-");
		return false;
	}

	highlightRegions(regions) {
		if (this.isAlreadySelected(regions)) {
			//this.onRegionHighlightStart();
			return;
		}

		this.clearRegionsAndPathways();
		this.clearLabels();

		if (this.layer1.selectAll("circle").size() > 1 || regions.length > 1) {
			this.clearCentroids();
		}

		if (!Array.isArray(regions)) {
			regions = [regions];
		}

		console.log("regions", regions);

		regions.forEach((region) => {
			if (!region) {
				return;
			}

			if (typeof region === "string") {
				region = this.regions[region];
			}
			console.log(region);

			// append centroid marker if needed
			const centroid = this.layer1.select("circle");

			if (centroid.empty() || regions.length > 1) {
				this.layer1
					.append("circle")
					.attr("data-interactive", true)
					.attr("cx", region.centroid.x)
					.attr("cy", region.centroid.y)
					.attr("r", 5)
					.attr("fill", "white");
			} else {
				centroid
					.transition()
					.duration(500)
					.attr("cx", region.centroid.x)
					.attr("cy", region.centroid.y);
			}

			// draw path if region has label data
			const self = this;
			if (region.label !== undefined) {
				const lineFunction = line()
					.x((d) => d.x)
					.y((d) => d.y);

				const sOffsetX = region.label.position.x < region.centroid.x ? -10 : 10;
				const sOffsetY = region.label.position.y < region.centroid.y ? -10 : 10;
				const eOffsetX = region.label.position.x < region.centroid.x ? 85 : -85;
				const eOffsetY = region.label.position.y < region.centroid.y ? 20 : -20;

				const startPoint = region.label.altDrawMode
					? { x: region.centroid.x, y: region.centroid.y + sOffsetY }
					: { x: region.centroid.x + sOffsetX, y: region.centroid.y };
				const midPoint = region.label.altDrawMode
					? { x: region.centroid.x, y: region.label.position.y }
					: { x: region.label.position.x, y: region.centroid.y };
				const endPoint = !region.label.altDrawMode
					? {
						x: region.label.position.x,
						y: region.label.position.y + eOffsetY,
					}
					: {
						x: region.label.position.x + eOffsetX,
						y: region.label.position.y,
					};

				const lineData = [startPoint, midPoint, endPoint];

				// build path and fade in
				const path = this.layer2
					.append("path")
					.attr("data-interactive", true)
					.attr("d", lineFunction(lineData))
					.attr("stroke-width", 2)
					.attr("stroke", "white")
					.attr("fill", "none");

				const totalLength = path.node().getTotalLength();

				path
					.attr("stroke-dasharray", `${totalLength} ${totalLength}`)
					.attr("stroke-dashoffset", totalLength)
					.transition()
					.delay(400)
					.duration(500)
					.ease(easeSinInOut)
					.attr("stroke-dashoffset", 0);
				// .on("end", repeat);

				// draw label
				this.layer3
					.append("rect")
					.attr("class", "region-label")
					.attr("width", 250)
					.attr("height", 30)
					.attr("x", region.label.position.x - 125)
					.attr("y", region.label.position.y - 15)
					.attr("opacity", 0)
					.transition()
					.duration(500)
					.delay(800)
					.attr("opacity", 1)
					.on("end", function () {
						self.onRegionHighlightEnd(region);
						return 1;
					});

				// draw text
				this.layer3
					.append("text")
					.attr("class", "region-label-text")
					.attr("x", region.label.position.x)
					.attr("y", region.label.position.y + 4)
					// .attr("alignment-baseline", "middle")
					.attr("text-anchor", "middle")
					.text(region.name)
					.attr("opacity", 0)
					.transition()
					.duration(200)
					.delay(1200)
					.attr("opacity", 1);
			}

			const polygon = select(region.polygon);
			if (polygon) {
				polygon.classed("active", true);
			}

			self.onRegionHighlightStart(region);
		});
	}

	highlightPathways(pathways) {
		if (this.isAlreadySelected(pathways)) {
			this.onRegionHighlightStart();
			return;
		}

		this.clearRegionsAndPathways();
		this.clearLabels();
		this.clearCentroids();

		if (!Array.isArray(pathways)) {
			pathways = [pathways];
		}

		pathways.forEach((pathway) => {
			if (!pathway) {
				return;
			}

			if (typeof pathway === "string") {
				pathway = this.pathways[pathway];
			}

			// draw label if region has label data
			const self = this;
			if (pathway.labels) {
				pathway.labels.forEach((label) => {
					const lineFunction = line()
						.x((d) => d.x)
						.y((d) => d.y);

					const defaults = {
						size: { width: 240, height: 32 },
						offset: { x: 0, y: 0 },
					};
					label = { ...defaults, ...label };

					// append centroid marker if needed
					if (Object.values(label.offset).some((x) => x !== 0) || !label.text) {
						this.layer1
							.append("circle")
							.attr("cx", label.position.x)
							.attr("cy", label.position.y)
							.attr("r", 5)
							.attr("fill", "white");
					}

					if (!label.text) {
						return;
					}

					const lineData = [
						label.position,
						{
							x: label.position.x + label.offset.x,
							y: label.position.y + label.offset.y,
						},
					];

					// build path and fade in
					const path = this.layer2
						.append("path")
						.attr("d", lineFunction(lineData))
						.attr("stroke-width", 2)
						.attr("stroke", "white")
						.attr("fill", "none");

					const totalLength = path.node().getTotalLength();

					path
						.attr("stroke-dasharray", `${totalLength} ${totalLength}`)
						.attr("stroke-dashoffset", totalLength)
						.transition()
						.delay(400)
						.duration(500)
						.ease(easeSinInOut)
						.attr("stroke-dashoffset", 0);

					// draw label
					this.layer3
						.append("rect")
						.attr("class", "region-label")
						.attr("width", label.size.width)
						.attr("height", label.size.height)
						.attr("x", label.position.x + label.offset.x - label.size.width / 2)
						.attr("y", label.position.y + label.offset.y - label.size.height / 2)
						.attr("opacity", 0)
						.transition()
						.duration(500)
						.delay(800)
						.attr("opacity", 1);

					// draw text
					this.layer3
						.append("text")
						.attr("class", "region-label-text")
						.attr("x", label.position.x + label.offset.x)
						.attr("y", label.position.y + label.offset.y + 4)
						// .attr("alignment-baseline", "middle")
						.attr("text-anchor", "middle")
						.text(label.text)
						.attr("opacity", 0)
						.transition()
						.duration(200)
						.delay(1200)
						.attr("opacity", 1);
				});
			}

			const path = select(pathway.path);
			path
				.attr("stroke", pathway.color)
				.attr("stroke-width", 0)
				.transition()
				.duration(500)
				.attr("stroke-width", pathway.width);

			if (pathway.glow) {
				path.style("filter", "url(#glow)");
			}

			self.onRegionHighlightStart(pathway);
		});
	}

	onRegionHighlightStart(region) { }
	onRegionHighlightEnd(region) { }
}

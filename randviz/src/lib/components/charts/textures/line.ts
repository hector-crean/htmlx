import { Orientation } from ".";
import rand from "./random";


interface PatternFunction {
	(selection: any): void;
	heavier: (multiplier?: number) => PatternFunction;
	lighter: (divider?: number) => PatternFunction;
	thinner: (multiplier?: number) => PatternFunction;
	thicker: (divider?: number) => PatternFunction;
	background: (color: string) => PatternFunction;
	size: (newSize: number) => PatternFunction;
	orientation: (...args: Orientation[]) => PatternFunction;
	shapeRendering: (rendering: string) => PatternFunction;
	stroke: (color: string) => PatternFunction;
	strokeWidth: (width: number) => PatternFunction;
	id: (newId?: string) => string | PatternFunction;
	url: () => string;
}

export default function lines(): PatternFunction {
	let size = 20;
	let stroke = "#343434";
	let strokeWidth = 2;
	let background = "";
	let id = rand();
	let orientation: Orientation[] = ["diagonal"];
	let shapeRendering = "auto";

	const path = (orientation: Orientation) => {
		const s = size;
		switch (orientation) {
			case "0/8":
			case "vertical":
				return `M ${s / 2}, 0 l 0, ${s}`;
			case "1/8":
				return `M ${-s / 4},${s} l ${s / 2},${-s} M ${s / 4},${s} l ${
					s / 2
				},${-s} M ${(s * 3) / 4},${s} l ${s / 2},${-s}`;
			case "2/8":
			case "diagonal":
				return `M 0,${s} l ${s},${-s} M ${-s / 4},${s / 4} l ${s / 2},${
					-s / 2
				} M ${(3 / 4) * s},${(5 / 4) * s} l ${s / 2},${-s / 2}`;
			case "3/8":
				return `M 0,${(3 / 4) * s} l ${s},${-s / 2} M 0,${s / 4} l ${s},${
					-s / 2
				} M 0,${(s * 5) / 4} l ${s},${-s / 2}`;
			case "4/8":
			case "horizontal":
				return `M 0,${s / 2} l ${s},0`;
			case "5/8":
				return `M 0,${-s / 4} l ${s},${s / 2}M 0,${s / 4} l ${s},${s / 2} M 0,${
					(s * 3) / 4
				} l ${s},${s / 2}`;
			case "6/8":
				return `M 0,0 l ${s},${s} M ${-s / 4},${(3 / 4) * s} l ${s / 2},${
					s / 2
				} M ${(s * 3) / 4},${-s / 4} l ${s / 2},${s / 2}`;
			case "7/8":
				return `M ${-s / 4},0 l ${s / 2},${s} M ${s / 4},0 l ${s / 2},${s} M ${
					(s * 3) / 4
				},0 l ${s / 2},${s}`;
			default:
				return `M ${s / 2}, 0 l 0, ${s}`;
		}
	};

	const $ = (selection: any) => {
		const group = selection
			.append("defs")
			.append("pattern")
			.attr("id", id)
			.attr("patternUnits", "userSpaceOnUse")
			.attr("width", size)
			.attr("height", size);

		if (background) {
			group
				.append("rect")
				.attr("width", size)
				.attr("height", size)
				.attr("fill", background);
		}

		for (const o of orientation) {
			group
				.append("path")
				.attr("d", path(o))
				.attr("stroke-width", strokeWidth)
				.attr("shape-rendering", shapeRendering)
				.attr("stroke", stroke)
				.attr("stroke-linecap", "square");
		}
	};

	$.heavier = function (multiplier = 2) {
		strokeWidth *= multiplier;
		return $;
	};

	$.lighter = function (divider = 2) {
		strokeWidth /= divider;
		return $;
	};

	$.thinner = function (multiplier = 2) {
		size *= multiplier;
		return $;
	};

	$.thicker = function (divider = 2) {
		size /= divider;
		return $;
	};

	$.background = function (color: string) {
		background = color;
		return $;
	};

	$.size = function (newSize: number) {
		size = newSize;
		return $;
	};

	$.orientation = function (...args: Orientation[]) {
		if (arguments.length === 0) {
			return $;
		}

		orientation = args;
		return $;
	};

	$.shapeRendering = function (rendering: string) {
		shapeRendering = rendering;
		return $;
	};

	$.stroke = function (color: string) {
		stroke = color;
		return $;
	};

	$.strokeWidth = function (width: number) {
		strokeWidth = width;
		return $;
	};

	$.id = function (newId?: string) {
		if (arguments.length === 0) {
			return id;
		}

		id = newId!;
		return $;
	};

	$.url = function () {
		return `url(#${id})`;
	};

	return $ as PatternFunction;
}

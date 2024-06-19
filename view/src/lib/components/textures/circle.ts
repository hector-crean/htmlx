import { PatternFunction } from ".";
import rand from "./random";



export default function circles(): PatternFunction {
	let size = 20;
	let background = "";
	let radius = 2;
	let complement = false;
	let fill = "#343434";
	let stroke = "#343434";
	let strokeWidth = 0;
	let id = rand();

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

		group
			.append("circle")
			.attr("cx", size / 2)
			.attr("cy", size / 2)
			.attr("r", radius)
			.attr("fill", fill)
			.attr("stroke", stroke)
			.attr("stroke-width", strokeWidth);

		if (complement) {
			for (const corner of [
				[0, 0],
				[0, size],
				[size, 0],
				[size, size],
			]) {
				group
					.append("circle")
					.attr("cx", corner[0])
					.attr("cy", corner[1])
					.attr("r", radius)
					.attr("fill", fill)
					.attr("stroke", stroke)
					.attr("stroke-width", strokeWidth);
			}
		}
	};

	$.heavier = function (multiplier = 2) {
		radius *= multiplier;
		return $;
	};

	$.lighter = function (divider = 2) {
		radius /= divider;
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

	$.complement = function (value = true) {
		complement = value;
		return $;
	};

	$.radius = function (newRadius: number) {
		radius = newRadius;
		return $;
	};

	$.fill = function (color: string) {
		fill = color;
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

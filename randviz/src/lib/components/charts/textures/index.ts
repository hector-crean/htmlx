import circles from './circle';
import lines from './line';
import paths from './path';

/* eslint import/no-anonymous-default-export: [2, {"allowObject": true}] */
export default {
	circles,
	lines,
	paths
};


type Orientation =
	| "0/8"
	| "1/8"
	| "2/8"
	| "3/8"
	| "4/8"
	| "5/8"
	| "6/8"
	| "7/8"
	| "vertical"
	| "diagonal"
	| "horizontal";



interface LinePatternFunction {
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


interface CirclePatternFunction {
	(selection: any): void;
	heavier: (multiplier?: number) => PatternFunction;
	lighter: (divider?: number) => PatternFunction;
	thinner: (multiplier?: number) => PatternFunction;
	thicker: (divider?: number) => PatternFunction;
	background: (color: string) => PatternFunction;
	size: (newSize: number) => PatternFunction;
	complement: (value?: boolean) => PatternFunction;
	radius: (newRadius: number) => PatternFunction;
	fill: (color: string) => PatternFunction;
	stroke: (color: string) => PatternFunction;
	strokeWidth: (width: number) => PatternFunction;
	id: (newId?: string) => string | PatternFunction;
	url: () => string;
}


type PathType =
	| "squares"
	| "nylon"
	| "waves"
	| "woven"
	| "crosses"
	| "caps"
	| "hexagons";



interface PathPatternFunction {
	(selection: any): void;
	heavier: (multiplier?: number) => PatternFunction;
	lighter: (divider?: number) => PatternFunction;
	thinner: (multiplier?: number) => PatternFunction;
	thicker: (divider?: number) => PatternFunction;
	background: (color: string) => PatternFunction;
	shapeRendering: (rendering: string) => PatternFunction;
	size: (newSize: number) => PatternFunction;
	d: (pathFunction: (s: number) => string | PathType) => PatternFunction;
	fill: (color: string) => PatternFunction;
	stroke: (color: string) => PatternFunction;
	strokeWidth: (width: number) => PatternFunction;
	id: (newId?: string) => string | PatternFunction;
	url: () => string;
}


type PatternFunction = PathPatternFunction | CirclePatternFunction | LinePatternFunction

export type { Orientation, PathType, PatternFunction };


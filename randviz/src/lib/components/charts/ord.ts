export interface Eq<A> {
    readonly equals: (x: A, y: A) => boolean
}

export type Ordering = -1 | 0 | 1

export interface Ord<A> extends Eq<A> {
    readonly compare: (first: A, second: A) => Ordering
}


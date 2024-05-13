export type BarChartProps = { title: string; bars: BarDatum[] }
export type BarDatum = { title: string; icon: string; content: string; percent: number }
export type Block = { type: "RichTextBlock"; data: RichTextProps } | { type: "TabsBlock"; data: TabsProps } | { type: "TextWithIconBlock"; data: TextWithIconProps } | { type: "InteractiveBrainBlock"; data: InteractiveBrainProps } | { type: "BarChartBlock"; data: BarChartProps }
export type BrainComment = { icon: string; symptom: string; highlightedRegions: BrainRegion[]; description: string }
export type BrainRegion = { name: BrainRegionName; fillColor: string; label: BrainRegionLabel; centroid: Vec2; points: Vec2[] }
export type BrainRegionLabel = { position: Vec2; altDrawMode: boolean }
export type BrainRegionName = "frontosubcortical" | "orbitofrontal" | "anteriorCingulatedGyrus" | "bilateralTemporalCortex" | "parietalLobe" | "thalamus" | "hippocampus" | "amygdala" | "hypothalamus" | "anteriorCingulateCortex" | "posteriorCingulateCortex" | "striatum" | "prefrontalCortex" | "ventralFrontalCortex" | "frontalLobe" | "dlpfc" | "vlpfc" | "nucleusAccumbens" | "basalForebrain" | "anteriorCaudate" | "greyMatter" | "lateralVentricle" | "occipitalLobe" | "auditoryCortex" | "substantiaNigra" | "nucleusAccumbensArea" | "amyloidStage1MildRegion1" | "amyloidStage2ModerateRegion1" | "amyloidStage2MildRegion1" | "amyloidStage3SevereRegion1" | "amyloidStage3ModerateRegion1" | "amyloidStage3ModerateRegion2" | "locusCoeruleus"
export type CommentGroup = { name: string; comments: BrainComment[] }
export type InteractiveBrainProps = { groups: CommentGroup[] }
export type RichText = { type: "Html"; data: string }
export type RichTextProps = { text: RichText }
export type Tab = { name: string; blocks: Block[] }
export type TabsProps = { id: string; tabs: Tab[] }
export type TextWithIconProps = { icon_src: string; rich_text: RichText }
export type Vec2 = { x: number; y: number }



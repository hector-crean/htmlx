export type Block = { type: "RichTextBlock"; data: RichTextProps } | { type: "TabsBlock"; data: TabsProps }
export type BlocksProps = { blocks: Block[] }
export type RichText = { type: "Html"; data: string }
export type RichTextProps = { text: RichText }
export type Tab = { name: string; tab_content: BlocksProps }
export type TabsProps = { id: string; tabs: Tab[] }

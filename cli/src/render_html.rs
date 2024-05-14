use blocks::block::bar_chart::BarChartProps;
use blocks::block::interactive_brain::InteractiveBrainProps;
use blocks::block::rich_text::{RichText, RichTextProps};
use blocks::block::tabs::{Tab, TabsProps};
use blocks::block::{Block, BlocksProps};
use blocks_macros::{rich_text, rich_text_block};
use color_eyre::eyre;
use std::fs::File;
use std::io::Write;
use uuid::Uuid;

pub fn example_blocks() -> Vec<Block> {
    vec![
        Block::InteractiveBrainBlock(InteractiveBrainProps::default()),
        Block::BarChartBlock(BarChartProps::default()),
        Block::TabsBlock(TabsProps {
            id: Uuid::new_v4(),
            tabs: vec![Tab {
                name: "Tab 1".to_string(),
                blocks: vec![
                    rich_text_block!(
                        "/Users/hectorcrean/Rust/htmlx/cli/src/input/mars-page/9c4f69e1-b8d4-4cfe-b88a-a4acdbea39e9.html"
                    ),
                    Block::InteractiveBrainBlock(InteractiveBrainProps::default()),
                    Block::BarChartBlock(BarChartProps::default()),
                    Block::TabsBlock(TabsProps {
                        id: Uuid::new_v4(),
                        tabs: vec![Tab {
                            name: "Tab 1".to_string(),
                            blocks: vec![Block::TabsBlock(TabsProps {
                                id: Uuid::new_v4(),
                                tabs: vec![],
                            })],
                        }],
                    }),
                ],
            }],
        }),
    ]
}

pub fn render_html() -> eyre::Result<()> {
    let blocks = BlocksProps::new(example_blocks());

    let code = blocks.render()?;

    let mut file = File::create("/Users/hectorcrean/Rust/htmlx/cli/src/outputs/rendered.html")?;

    file.write(code.as_bytes())?;

    Ok(())
}

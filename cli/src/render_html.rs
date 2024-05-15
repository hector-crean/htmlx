use crate::PROJECT_ROOT;
use blocks::block::bar_chart::BarChartProps;
use blocks::block::interactive_brain::InteractiveBrainProps;
use blocks::block::rich_text::{RichText, RichTextProps};
use blocks::block::tabs::{Tab, TabsProps};
use blocks::block::{Block, BlocksProps};
use color_eyre::eyre;
use std::fs::File;
use std::io::Write;
use std::path::{Path, PathBuf};
use uuid::Uuid;

#[macro_export]
macro_rules! rich_text_block {
    ($file_path:expr) => {
        Block::RichTextBlock(RichTextProps {
            text: RichText::Html(include_str!($file_path).to_string()),
        })
    };
}


pub trait Pagelike {
    fn blocks() -> Vec<Block>;
    fn output_path(&self) -> PathBuf;
    fn render_html(&self) -> eyre::Result<()> {
        let blocks = BlocksProps::new(Self::blocks());
    
        let code = blocks.render()?;
    
        let mut file = File::create(self.output_path())?;
    
        file.write(code.as_bytes())?;
    
        Ok(())
    }
    
}

pub struct PTSDSymptomsNode {
    output_path: PathBuf
}
impl PTSDSymptomsNode {
    pub fn new(output_path: PathBuf) -> Self {
        Self { output_path}
    }
}

impl Pagelike for PTSDSymptomsNode {
    fn output_path(&self) -> PathBuf {
        self.output_path.clone()
    }
    fn blocks() -> Vec<Block> {
        vec![
            Block::InteractiveBrainBlock(InteractiveBrainProps::default()),
            Block::BarChartBlock(BarChartProps::default()),
            Block::TabsBlock(TabsProps {
                id: Uuid::new_v4(),
                tabs: vec![
                    Tab {
                        name: "Tab 1".to_string(),
                        blocks: vec![
                            rich_text_block!(
                                "./input/mars-page/9c4f69e1-b8d4-4cfe-b88a-a4acdbea39e9.html"
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
                    },
                    Tab {
                        name: "Tab 1".to_string(),
                        blocks: vec![
                            rich_text_block!(
                                "./input/mars-page/9c4f69e1-b8d4-4cfe-b88a-a4acdbea39e9.html"
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
                    },
                ],
            }),
        ]
    }
}


pub fn render_html() -> eyre::Result<()> {
    let blocks = BlocksProps::new(PTSDSymptomsNode::blocks());

    let code = blocks.render()?;

    let path: PathBuf = [PROJECT_ROOT, "src", "outputs", "output.html"]
        .iter()
        .collect();

    println!("{}", &path.display());

    let mut file = File::create(path)?;

    file.write(code.as_bytes())?;

    Ok(())
}

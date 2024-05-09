
use core::fmt;
use std::fmt::{write, Display, Write};

use askama::Template;
use rich_text::{RichText, RichTextTemplate};
use tabs::{Tab, TabsTemplate};

pub mod rich_text;
pub mod tabs;


#[derive(Clone)]
pub struct BlocksTemplate {
    blocks: Vec<Block>
}

impl fmt::Display for BlocksTemplate {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for block in &self.blocks {
            writeln!(f, "{:#}", block)?;
        }
        fmt::Result::Ok(())
    }
}

impl BlocksTemplate {
    pub fn render(&self) -> Result<String, std::fmt::Error> {
        let mut buf = String::new();
       
        write!(&mut buf, "{}", self)?;

        Ok(buf)
    }
}

#[derive(Clone)]
pub enum Block {
    RichTextBlock(RichTextTemplate),
    TabsBlock(TabsTemplate<'static>)
}


impl fmt::Display for Block {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Block::RichTextBlock(rich_text) => {
                let rendered = rich_text.render().map_err(|_| fmt::Error)?;
                write!(f, "{}", rendered)
            },
            Block::TabsBlock(tabs) => {
                let rendered = tabs.render().map_err(|_| fmt::Error)?;
                write!(f, "{}", rendered)
            }
        }
    }
}

impl BlocksTemplate {
    pub fn example_blocks() -> Self {
        Self {
            blocks: vec![ 
                Block::TabsBlock(
                    TabsTemplate { 
                        id: "#tabs-outer", 
                        tabs: vec![ 
                                Tab { 
                                    name: "Tab 1".to_string(), 
                                    tab_content: BlocksTemplate { 
                                        blocks:  vec![
                                            Block::TabsBlock(
                                                TabsTemplate { 
                                                    id: "#tabs-inner", 
                                                    tabs: vec![ 
                                                        Tab { 
                                                            name: "Tab 1".to_string(), 
                                                            tab_content: BlocksTemplate { 
                                                                blocks:  vec![
                                                                    Block::TabsBlock(
                                                                        TabsTemplate { 
                                                                            id: "#tabs-inner", 
                                                                            tabs: vec![ 
                                                                                Tab { 
                                                                                    name: "Tab 1".to_string(), 
                                                                                    tab_content: BlocksTemplate { 
                                                                                        blocks:  vec![
                                                                                            Block::TabsBlock(
                                                                                                TabsTemplate { 
                                                                                                    id: "#tabs-inner", 
                                                                                                    tabs: vec![ 
                                                                                                            
                                                                                                        ]}
                                                                                            )
                                                                                        ]
                                                                                    }
                                                                                }
                                                                                ]}
                                                                    )
                                                                ]
                                                            }
                                                        },
                                                        Tab { 
                                                            name: "Tab 1".to_string(), 
                                                            tab_content: BlocksTemplate { 
                                                                blocks:  vec![
                                                                    Block::TabsBlock(
                                                                        TabsTemplate { 
                                                                            id: "#tabs-inner", 
                                                                            tabs: vec![ 
                                                                                    
                                                                                ]}
                                                                    )
                                                                ]
                                                            }
                                                        } 
                                                        ]}
                                            )
                                        ]
                                    }
                                }
                            ]}
                )
            ]
        }
    } 
}

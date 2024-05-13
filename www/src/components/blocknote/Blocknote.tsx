"use client";

import "@blocknote/core/fonts/inter.css";
import {
    BasicTextStyleButton,
    BlockNoteView,
    BlockTypeSelect,
    ColorStyleButton,
    FormattingToolbar,
    FormattingToolbarController,
    ImageCaptionButton,
    NestBlockButton,
    ReplaceImageButton,
    TextAlignButton,
    UnnestBlockButton,
} from "@blocknote/react";
import "@blocknote/react/style.css";
import './Blocks.css';

import { Block, BlockNoteEditor } from "@blocknote/core";
import { DialogTrigger } from "@radix-ui/react-dialog";
import { useCallback, useEffect, useMemo, useState } from "react";
import { Button } from "../ui/button";
import { Dialog, DialogClose, DialogContent, DialogFooter } from "../ui/dialog";
import { blocknoteSchema } from "./spans";




type EditorMode = 'edit' | 'review'





interface RichTextEditorProps {
    blocks: Array<Block>;
    setBlocks: (blocks: Array<Block>) => void;
}

const InlineToolbar = () => (

    <FormattingToolbarController
        formattingToolbar={(props) => (
            <FormattingToolbar>
                <BlockTypeSelect key={"blockTypeSelect"} />
                <ImageCaptionButton key={"imageCaptionButton"} />
                <ReplaceImageButton key={"replaceImageButton"} />

                <BasicTextStyleButton
                    basicTextStyle={"bold"}
                    key={"boldStyleButton"}
                />
                <BasicTextStyleButton
                    basicTextStyle={"italic"}
                    key={"italicStyleButton"}
                />
                <BasicTextStyleButton
                    basicTextStyle={"underline"}
                    key={"underlineStyleButton"}
                />
                <BasicTextStyleButton
                    basicTextStyle={"strike"}
                    key={"strikeStyleButton"}
                />
                {/* Extra button to toggle code styles */}
                <BasicTextStyleButton
                    key={"codeStyleButton"}
                    basicTextStyle={"code"}
                />

                <TextAlignButton
                    textAlignment={"left"}
                    key={"textAlignLeftButton"}
                />
                <TextAlignButton
                    textAlignment={"center"}
                    key={"textAlignCenterButton"}
                />
                <TextAlignButton
                    textAlignment={"right"}
                    key={"textAlignRightButton"}
                />

                <ColorStyleButton key={"colorStyleButton"} />

                <NestBlockButton key={"nestBlockButton"} />
                <UnnestBlockButton key={"unnestBlockButton"} />


            </FormattingToolbar>
        )} />
)

const RichTextEditor = ({ blocks, setBlocks }: RichTextEditorProps) => {


    const editor = useMemo(() => {
        return BlockNoteEditor.create({ initialContent: blocks, schema: blocknoteSchema });
    }, []);


    const [draft, setDraft] = useState(blocks);

    const [html, setHtml] = useState('')

    const onChange = useCallback(() => {
        setDraft(editor.document ?? []);
    }, [editor]);

    // Handler to open edit mode


    // Handler to close edit mode and save changes
    const handleSave = useCallback(() => {
        setBlocks(draft);


    }, [setBlocks, draft])

    useEffect(() => {
        const updateHtml = async () => {
            const html = await editor.blocksToHTMLLossy();
            setHtml(html)
        }

        updateHtml()
    }, [draft, editor])


    return (
        <>

            <Dialog>
                <DialogTrigger asChild>
                    <Button variant="outline" className="absolute top-0 right-0">Edit</Button>
                </DialogTrigger>
                <DialogContent>
                    <div className='flex flex-col'>
                        <div className="flex flex-row w-full h-full">
                            <div className='flex flex-col h-full flex-1'>
                                <BlockNoteView
                                    editable={true}
                                    editor={editor}
                                    formattingToolbar={false}
                                    className='relative max-w-[900px]'
                                    onChange={onChange}
                                >
                                    <InlineToolbar />
                                </BlockNoteView>
                            </div>
                        </div>
                    </div>
                    <DialogFooter>
                        <DialogClose asChild>
                            <Button onClick={handleSave} type="submit">Save</Button>
                        </DialogClose>
                    </DialogFooter>
                </DialogContent>
            </Dialog>

            <div dangerouslySetInnerHTML={{ __html: html }}></div>






        </>


    );
}


export { RichTextEditor };









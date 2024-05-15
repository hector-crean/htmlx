"use client";

import { MoonIcon, SunIcon } from "@/icons";
import { Presence, RoomEvent, Storage, UserMeta, useRoom, useSelf, useThreads } from "@/liveblocks-configs/block-room.config";
import { BlockNoteEditor, filterSuggestionItems, getDefaultSlashMenuItems } from "@blocknote/core";
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
  SuggestionMenuController,
  TextAlignButton,
  UnnestBlockButton,
  useCreateBlockNote
} from "@blocknote/react";
import "@blocknote/react/style.css";
import LiveblocksProvider from "@liveblocks/yjs";
import { saveAs } from 'file-saver';
import JSZip from 'jszip';
import { useCallback, useEffect, useState } from "react";
import * as Y from "yjs";
import { InboxPopover } from "../inbox/Inbox";
import { Button } from "../ui/button";
import { Avatars } from "./Avatars";
import { ThreadButton, ThreadComposer } from "./ThreadButton";
import { blocknoteSchema } from "./schema";


''



enum EditorMode {
  Edit = 0,
  Review = 1
}



type EditorState =
  | {
    mode: EditorMode.Edit;
  }
  | {
    mode: EditorMode.Review;

  }


// Collaborative text editor with simple rich text, live cursors, and live avatars
export function CollaborativeEditor() {

  const room = useRoom();

  const [doc, setDoc] = useState<Y.Doc>();
  const [provider, setProvider] = useState<LiveblocksProvider<Presence, Storage, UserMeta, RoomEvent>>();

  // Set up Liveblocks Yjs provider
  useEffect(() => {
    const yDoc = new Y.Doc();
    const yProvider = new LiveblocksProvider(room, yDoc);
    setDoc(yDoc);
    setProvider(yProvider);

    return () => {
      yDoc?.destroy();
      yProvider?.destroy();
    };
  }, [room]);

  if (!doc || !provider) {
    return null;
  }

  return <BlockNote roomId={room.id} doc={doc} provider={provider} />;
}

type EditorProps = {
  doc: Y.Doc;
  provider: LiveblocksProvider<Presence, Storage, UserMeta, RoomEvent>;
  roomId: string
};

function BlockNote({ doc, provider, roomId }: EditorProps) {
  // Get user info from Liveblocks authentication endpoint
  const userInfo = useSelf((me) => me.info);




  // @ts-ignore
  const editor: BlockNoteEditor = useCreateBlockNote({
    schema: blocknoteSchema,
    collaboration: {
      provider,
      // Where to store BlockNote data in the Y.Doc:
      fragment: doc.getXmlFragment("document-store"),
      // Information for this user:
      user: {
        name: userInfo.name ?? '',
        color: userInfo.color as string ?? 'red',
      },


    },

  });


  const [theme, setTheme] = useState<"light" | "dark">("light");

  const changeTheme = useCallback(() => {
    const newTheme = theme === "light" ? "dark" : "light";
    document.documentElement.setAttribute("data-theme", newTheme);
    setTheme(newTheme);
  }, [theme]);


  const { threads } = useThreads();


  const handleExport = () => {

    const zip = new JSZip();



    const blockOrder: Array<string> = []


    console.log(editor.document)

    editor.document.forEach((block) => {

      const html = editor.blocksToHTMLLossy([block]);
      const filename = `${block.id}.html`

      switch (block.type) {
        default: {
          blockOrder.push(filename)
          zip.file(filename, html);
        }
      }

    })

    // editor.forEachBlock((block) => {

    //   const html = editor.blocksToHTMLLossy([block]);
    //   const filename = `${block.id}.html`

    //   switch (block.type) {
    //     default: {
    //       blockOrder.push(filename)
    //       zip.file(filename, html);
    //     }
    //   }


    //   return true

    // });
    const metadata = { blockOrder: blockOrder, datetime: new Date().getUTCDate() }

    zip.file('metadata.json', JSON.stringify(metadata));



    zip.generateAsync({ type: 'blob' }).then((content) => {
      saveAs(content, `${roomId}.zip`);
    });



  }


  return (
    <div className='flex flex-col bg-white absolute top-0 right-0 left-0 bottom-0'>
      <div className='top-0 right-0 left-0 bottom-0 flex flex-grow-0 flex-shrink-0 justify-between items-start bg-slate-50 p-4 z-50'>
        <Button onClick={handleExport}>
          Export
        </Button>
        <Button
          variant='default'
          onClick={changeTheme}
          aria-label="Switch Theme"
        >
          {theme === "dark" ? (
            <SunIcon style={{ width: "18px" }} />
          ) : (
            <MoonIcon style={{ width: "18px" }} />
          )}
        </Button>
        <InboxPopover />
        <Avatars />
      </div>
      <div className="flex flex-row w-full h-full">
        <div className='flex flex-col h-full flex-1'>
          <BlockNoteView
            editor={editor}
            slashMenu={false}
            formattingToolbar={false}
            className='relative min-h-full max-w-[900px]'
            theme={theme}

          >
            {/* Replaces the default Slash Menu. */}
            <SuggestionMenuController
              triggerCharacter={"/"}
              getItems={async (query) =>
                // Gets all default slash menu items and `insertAlert` item.
                filterSuggestionItems(
                  [...getDefaultSlashMenuItems(editor)],
                  query
                )
              }
            />
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

                  <ThreadButton key={'threadButton'} />

                </FormattingToolbar>
              )} />
          </BlockNoteView>
        </div>
        <div className="flex flex-col h-full bg-gray-900  items-start justify-start gap-2 p-2 overflow-y-scroll">
          <>
            {threads.map((thread) => (
              <ThreadComposer key={thread.id} thread={thread} />
            ))}
          </>


        </div>
      </div>
    </div>
  );
}

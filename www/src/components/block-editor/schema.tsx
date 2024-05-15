import {
    BlockNoteSchema,
    defaultBlockSpecs,
    defaultStyleSpecs,
    insertOrUpdateBlock
} from "@blocknote/core";
import { RiAlertFill, RiTable2 } from "react-icons/ri";
import { InteractiveBrain } from './blocks/InteractiveBrain';
import { KanbanBlock } from "./blocks/Kanban";
import { ThreadSpan } from "./spans/ThreadSpan";


const blocknoteSchema = BlockNoteSchema.create({
    styleSpecs: {
        // enable the default styles if desired
        ...defaultStyleSpecs,

        // Add your own custom style:
        thread: ThreadSpan,
    },
    blockSpecs: {
        ...defaultBlockSpecs,
        interactiveBrain: InteractiveBrain,
        kanban: KanbanBlock
    },

});



// Slash menu item to insert an Alert block
const insertInteractiveBrain = (editor: typeof blocknoteSchema.BlockNoteEditor) => ({
    title: "Alert",
    onItemClick: () => {
        insertOrUpdateBlock(editor, {
            type: 'interactiveBrain',
        });
    },
    aliases: [
        "alert",
        "notification",
        "emphasize",
        "warning",
        "error",
        "info",
        "success",
    ],
    group: "Other",
    icon: <RiAlertFill />,
});

const insertKanban = (editor: typeof blocknoteSchema.BlockNoteEditor) => ({
    title: "Kanban",
    onItemClick: () => {
        insertOrUpdateBlock(editor, {
            type: 'kanban',
            props: {
                cols: []
            }
        });
    },
    aliases: [

    ],
    group: "Other",
    icon: <RiTable2 />,
});


export { blocknoteSchema, insertInteractiveBrain, insertKanban };

import { BlockNoteSchema, defaultStyleSpecs } from "@blocknote/core";
import { ThreadSpan } from "./ThreadSpan";

const blocknoteSchema = BlockNoteSchema.create({
    styleSpecs: {
        // enable the default styles if desired
        ...defaultStyleSpecs,

        // Add your own custom style:
        thread: ThreadSpan,
    },
});

export { blocknoteSchema };

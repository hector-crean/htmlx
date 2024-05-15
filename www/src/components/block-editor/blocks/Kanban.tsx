import { KanbanBoard } from "@/components/kanban/KanbanBoard";
import { createReactBlockSpec } from "@blocknote/react";



const KanbanBlock = createReactBlockSpec(
    {
        type: "kanban",
        propSchema: {


        },
        content: 'none',
    },
    {
        render: ({ contentRef, editor, block }) => {


            return (
                <div className='flex-grow' >
                    <KanbanBoard initialCols={[]} />
                </div>
            );
        },
    }
);


export { KanbanBlock };


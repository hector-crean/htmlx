import { KanbanBoard } from "@/components/kanban/KanbanBoard";
import { createReactBlockSpec } from "@blocknote/react";



const KanbanBlock = createReactBlockSpec(
    {
        type: "kanban",
        propSchema: {
            cols: {
                default: []
            }

        },
        content: 'none',
    },
    {
        render: ({ contentRef, editor, block }) => {


            return (
                <div className='flex-grow' >
                    <KanbanBoard initialCols={block.props.cols} />
                </div>
            );
        },
    }
);


export { KanbanBlock };

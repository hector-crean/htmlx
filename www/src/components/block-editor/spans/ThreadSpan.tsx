import { createReactStyleSpec } from "@blocknote/react";

export const ThreadSpan = createReactStyleSpec(
    {
        type: 'Thread',
        propSchema: "string",
    },
    {
        render: (props) => (
            <span style={{ fontFamily: props.value }} ref={props.contentRef} />
        ),
    }
);
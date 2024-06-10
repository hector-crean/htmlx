import {
    SideMenuButton,
    SideMenuProps
} from "@blocknote/react";
import { CopyIcon } from "lucide-react";


async function copyToClipboard(text: string): Promise<void> {
    try {
        await navigator.clipboard.writeText(text);
        console.log('Text copied to clipboard');
    } catch (err) {
        console.error('Failed to copy text: ', err);
    }
}



// Custom Side Menu button to remove the hovered block.
const BlockUUID = (props: SideMenuProps) => {


    return (
        <SideMenuButton>
            <CopyIcon onClick={() => copyToClipboard(props.block.id)} />
        </SideMenuButton>
    );
}


export { BlockUUID };


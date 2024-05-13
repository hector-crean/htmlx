import { ReactFlowJsonObject, useReactFlow } from '@xyflow/react';
import { useCopyToClipboard } from 'usehooks-ts';

import { FlowEdgeType, FlowNodeType } from '@/components/flow/types';
import { ClipboardCopyIcon, ReloadIcon } from '@radix-ui/react-icons';
import { useCallback, useState } from 'react';
import { JSONTree } from 'react-json-tree';
import styles from './Sidebar.module.css';

const Sidebar = () => {

    const instance = useReactFlow<FlowNodeType, FlowEdgeType>()

    const [draftFlow, setDraftFlow] = useState<ReactFlowJsonObject<FlowNodeType, FlowEdgeType>>(instance.toObject())


    const handleExport = useCallback(() => {
        setDraftFlow(instance.toObject)
    }, [setDraftFlow, instance])


    const [copiedText, copyToClipboard] = useCopyToClipboard();

    const handleCopy = useCallback((text: string) => {

        copyToClipboard(text)
            .then(success => {
                if (success) {
                    console.log(`Text "${text}" copied to clipboard successfully.`);
                } else {
                    console.error('Failed to copy text to clipboard.');
                }
            });
    }, [copyToClipboard])



    return (
        <div className={styles.sidebar_container}>
            <JSONTree data={draftFlow} />
            <div className={styles.controls}>
                <ReloadIcon className={styles.icon} onPointerDown={handleExport} />
                <ClipboardCopyIcon className={styles.icon} onPointerDown={() => handleCopy(JSON.stringify(draftFlow))} /></div>

        </div>
    )
}

export { Sidebar };


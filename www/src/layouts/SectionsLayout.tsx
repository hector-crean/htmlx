import { ResizeContainer } from '@/components/resize-container/ResizeContainer'
import { ReactNode } from 'react'
import styles from './SectionsLayout.module.css'


const renderSections = (width: number, height: number, sections: Array<ReactNode>) => sections.map((section, idx) => {

    return (<section key={`section-${idx}`} className={styles.section} style={{ minHeight: `${height}px` }}>{section}</section>)
})



interface SectionsLayoutProps {
    children: Array<ReactNode>
}

const SectionsLayout = ({ children }: SectionsLayoutProps) => {



    return (

        <ResizeContainer as='div' className={styles.sections}>
            {({ width, height }) => renderSections(width, height, children)}
        </ResizeContainer>

    )
}



export { SectionsLayout }


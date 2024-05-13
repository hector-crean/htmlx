import { AnimatePresence, motion, useCycle } from "framer-motion";
import { ComponentProps, ReactNode, useState } from "react";
import styles from "./MainLayout.module.css";

interface MainLayoutProps extends ComponentProps<typeof motion.div> {
  header: ReactNode;
  main: ReactNode;
  overlay: ReactNode;
  sidebar: ReactNode;
  footer: ReactNode;
  cursors: ReactNode;
}

const MainLayout = ({
  header,
  main,
  overlay,
  sidebar,
  footer,
  cursors,
  ...props
}: MainLayoutProps) => {
  const [footerExpanded, setFooterExpanded] = useState(false);

  const [headerExpanded, setHeaderExpanded] = useState(false);

  const [sidebarOpen, cycleSidebarOpen] = useCycle(false, true);

  return (
    <motion.div className={styles.page_grid_container}>
      <motion.header
        className={styles.header_container}
        data-expanded={headerExpanded}
        onPointerDown={() => setHeaderExpanded(!headerExpanded)}
        layout
        transition={{
          type: "spring",
          stiffness: 200,
          damping: 30,
        }}
      >
        {header}
      </motion.header>

      <motion.main
        className={styles.main_container}
        layout
        transition={{
          type: "spring",
          stiffness: 700,
          damping: 30,
        }}
        {...props}
      >
        <div className={styles.main_content}>
          <motion.div className={styles.toggle_sidebar_btn}>
            {/* <DrawPathIcon
            onTap={() => cycleSidebarOpen()}
            path={"M2.14645 11.1464C1.95118 11.3417 1.95118 11.6583 2.14645 11.8536C2.34171 12.0488 2.65829 12.0488 2.85355 11.8536L6.85355 7.85355C7.04882 7.65829 7.04882 7.34171 6.85355 7.14645L2.85355 3.14645C2.65829 2.95118 2.34171 2.95118 2.14645 3.14645C1.95118 3.34171 1.95118 3.65829 2.14645 3.85355L5.79289 7.5L2.14645 11.1464ZM8.14645 11.1464C7.95118 11.3417 7.95118 11.6583 8.14645 11.8536C8.34171 12.0488 8.65829 12.0488 8.85355 11.8536L12.8536 7.85355C13.0488 7.65829 13.0488 7.34171 12.8536 7.14645L8.85355 3.14645C8.65829 2.95118 8.34171 2.95118 8.14645 3.14645C7.95118 3.34171 7.95118 3.65829 8.14645 3.85355L11.7929 7.5L8.14645 11.1464Z"}
          /> */}
          </motion.div>
          <div className={styles.cursors_container}>{cursors}</div>
          <div className={styles.main_content_inner}>{main}</div>
        </div>
        <AnimatePresence>
          {sidebarOpen && (
            <motion.aside
              className={styles.sidepanel}
              initial={{ maxWidth: 0 }}
              animate={{ maxWidth: "100%" }}
              exit={{
                maxWidth: 0,
                transition: { delay: 0, duration: 0.3 },
              }}
            >
              {sidebar}
            </motion.aside>
          )}
        </AnimatePresence>
      </motion.main>

      <motion.footer
        className={styles.footer_container}
        data-expanded={footerExpanded}
        // onPointerDown={() => setFooterExpanded(!footerExpanded)}
        layout
        transition={{
          type: "spring",
          stiffness: 200,
          damping: 30,
        }}
      >
        {footer}
      </motion.footer>
    </motion.div>
  );
};

export { MainLayout };

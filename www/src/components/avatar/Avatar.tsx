import * as RadixAvatar from '@radix-ui/react-avatar';
import * as Tooltip from '@radix-ui/react-tooltip';

import styles from './Avatar.module.css';


function formatNameAsInitials(fullName: string): string {
    // Split the full name into parts based on spaces
    const nameParts = fullName.trim().split(/\s+/);

    // Map each part of the name to its first character (uppercase) and join them together
    const initials = nameParts.map(name => name[0].toUpperCase()).join('');

    return initials;
}


type AvatarProps = {
    alt: string;
    src: string;
    color: string
}

const Avatar = ({ alt, src, color }: AvatarProps) => (
    <Tooltip.Provider>
        <Tooltip.Root>
            <Tooltip.Trigger asChild>
                <RadixAvatar.Root key={alt} className={styles.AvatarRoot} style={{ '--ring-color': color }}>
                    <RadixAvatar.Image
                        className={styles.AvatarImage}
                        src={src}
                        alt={alt}
                    />
                    <RadixAvatar.Fallback className={styles.AvatarFallback} delayMs={600}>
                        {formatNameAsInitials(alt)}
                    </RadixAvatar.Fallback>
                </RadixAvatar.Root>
            </Tooltip.Trigger>

            <Tooltip.Content side="top" className={styles.TooltipContent}>
                {alt}
                <Tooltip.Arrow className={styles.TooltipArrow} />
            </Tooltip.Content >
        </Tooltip.Root>
    </Tooltip.Provider>
)


export { Avatar };

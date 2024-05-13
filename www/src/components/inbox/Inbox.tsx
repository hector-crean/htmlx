import { useInboxNotifications, useMarkAllInboxNotificationsAsRead, useUnreadInboxNotificationsCount } from "@/liveblocks-configs/client";
import { ClientSideSuspense } from "@liveblocks/react";
import { InboxNotification, InboxNotificationList, } from "@liveblocks/react-comments";
import { ComponentProps, ComponentPropsWithoutRef, useCallback, useMemo, useState } from "react";
import { ErrorBoundary } from "react-error-boundary";
import { LiveblocksLink } from "../LiveblocksLink";
import { Case, Default, Switch } from "../Switch";
import { Loading } from "../loading/Loading";
import { Button } from "../ui/button";
import { Popover, PopoverContent, PopoverTrigger } from "../ui/popover";

interface InboxProps extends ComponentPropsWithoutRef<"ol"> {
  onNotificationClick?: () => void;
}



const Inbox = ({ className, onNotificationClick, ...props }: InboxProps) => {
  const { inboxNotifications } = useInboxNotifications();

  const notifcationsLen = useMemo(() => inboxNotifications?.length ?? 0, [inboxNotifications])


  return (
    <Switch>
      <Case condition={notifcationsLen <= 0}>
        <div className="w-full flex place-content-center place-items-center p-4 flex-1">There aren’t any notifications yet.</div>
      </Case>
      <Default>
        <InboxNotificationList className={'shadow-sm'} {...props}>
          {inboxNotifications?.map((inboxNotification) => {
            return (
              <InboxNotification
                key={inboxNotification.id}
                inboxNotification={inboxNotification}
                components={{ Anchor: LiveblocksLink }}
                onClick={onNotificationClick}
              />
            );
          })}
        </InboxNotificationList>
      </Default>
    </Switch>
  )
}

function InboxPopoverUnreadCount() {
  const { count } = useUnreadInboxNotificationsCount();

  return count ? <div
    className="absolute top-0 right-0 transform translate-x-1/2 -translate-y-1/2 flex place-content-center place-items-center bg-accent text-white text-xs font-semibold h-4 min-w-4 px-1 rounded-full"
  >{count}</div> : null;
}



export function InboxPopover({
  className,
  ...props
}: ComponentProps<typeof PopoverContent>) {
  const [isOpen, setOpen] = useState(false);
  const markAllInboxNotificationsAsRead = useMarkAllInboxNotificationsAsRead();

  const closePopover = useCallback(() => {
    setOpen(false);
  }, []);

  return (
    <Popover open={isOpen} onOpenChange={setOpen}>
      <PopoverTrigger asChild>
        <Button>
          <ErrorBoundary fallback={null}>
            <ClientSideSuspense fallback={null}>
              {() => <InboxPopoverUnreadCount />}
            </ClientSideSuspense>
          </ErrorBoundary>
          <svg
            width="20"
            height="20"
            fill="none"
            xmlns="http://www.w3.org/2000/svg"
          >
            <path
              d="m3.6 9.8 1.9-4.6A2 2 0 0 1 7.3 4h5.4a2 2 0 0 1 1.8 1.2l2 4.6V13a2 2 0 0 1-2 2h-9a2 2 0 0 1-2-2V9.8Z"
              stroke="currentColor"
              strokeWidth="1.5"
              strokeLinejoin="round"
            />
            <path
              d="M3.5 10h3c.3 0 .6.1.8.4l.9 1.2c.2.3.5.4.8.4h2c.3 0 .6-.1.8-.4l.9-1.2c.2-.3.5-.4.8-.4h3"
              stroke="currentColor"
              strokeWidth="1.5"
              strokeLinejoin="round"
            />
          </svg>
        </Button>
      </PopoverTrigger>

      <PopoverContent
        className="bg-white w-460 h-560 max-h-[calc(100vh-var(--header-height)-10px)] flex flex-col outline-none overflow-y-auto rounded-[0.75rem] shadow-border"
        collisionPadding={16}
        sideOffset={8}
        {...props}
      >
        <div className="sticky top-0 z-10 flex items-center justify-between bg-white w-full px-2 py-2 shadow-sm">
          <span>Notifications</span>
          <Button
            onClick={markAllInboxNotificationsAsRead}
          >
            Mark all as read
          </Button>
        </div>
        <ErrorBoundary
          fallback={
            <div className="error">
              There was an error while getting notifications.
            </div>
          }
        >
          <ClientSideSuspense fallback={<Loading />}>
            {() => <Inbox onNotificationClick={closePopover} />}
          </ClientSideSuspense>
        </ErrorBoundary>
      </PopoverContent>
    </Popover>
  );
}

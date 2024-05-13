import { useOthers, useSelf } from "@/liveblocks-configs/block-room.config";

const Avatars = () => {
  const users = useOthers();
  const currentUser = useSelf();

  return (
    <div className='flex p-2'>
      {users.map(({ connectionId, info }) => {
        return (
          <Avatar key={connectionId} picture={info.picture} name={info.name} />
        );
      })}

      {currentUser && (
        <div className="relative ml-8 first:ml-0">
          <Avatar
            picture={currentUser.info.picture}
            name={currentUser.info.name}
          />
        </div>
      )}
    </div>
  );
}

const Avatar = ({ picture, name }: { picture: string; name: string }) => {
  return (
    <div className='flex shrink-0 place-content-center relative rounded-full w-9 h-9 bg-gray-500 -ml-3'>
      <img
        src={picture}
        className={'w-full h-full rounded-full'}
      />
    </div>
  );
}


export { Avatar, Avatars };


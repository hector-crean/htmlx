import { CollaborativeEditor } from "@/components/block-editor/CollaborativeEditor";


interface TextRoomProps {
  id: string;
}
const TextRoom = ({ id }: TextRoomProps) => {

  return (
    <CollaborativeEditor />

  );
};

export { TextRoom };


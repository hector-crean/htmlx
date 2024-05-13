import { CollaborativeEditor } from "@/components/richt-text-editor/CollaborativeEditor";


interface TextRoomProps {
  id: string;
}
const TextRoom = ({ id }: TextRoomProps) => {

  return (
    <CollaborativeEditor />

  );
};

export { TextRoom };


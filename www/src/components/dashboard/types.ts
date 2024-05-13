import { liveblocks } from "@/app/api/liveblocks";

type RoomsQueryResult = Awaited<ReturnType<typeof liveblocks.getRooms>>;

const docFilters = {
    'all': {
        id: 'all'
    },
    'active': {
        id: 'active'
    },
    'draft': {
        id: 'draft'
    },
    'archived': {
        id: 'archived'
    }
} as const



type DocumentFilter = keyof typeof docFilters;


export { docFilters };
export type { DocumentFilter, RoomsQueryResult };


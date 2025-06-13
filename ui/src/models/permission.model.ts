import { type RecordId } from './record-id.model';

export interface Permission {
    id: RecordId;
    value: string;
    description: string;
}
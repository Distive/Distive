import type { Principal } from '@dfinity/principal';
import type { ActorMethod } from '@dfinity/agent';

export interface comment_input {
  'channel_id' : string,
  'parent_id' : [] | [string],
  'message' : string,
  'comment_id' : string,
}
export interface comment_output {
  'id' : string,
  'content' : string,
  'metadata' : Array<metadata_output>,
  'created_at' : bigint,
  'user_id' : string,
  'replies' : page,
}
export interface delete_comment_param {
  'channel_id' : string,
  'comment_id' : string,
}
export interface get_thread_param {
  'channel_id' : string,
  'cursor' : [] | [string],
  'metadata_user_ids' : Array<string>,
  'limit' : number,
}
export type metadata_output = [string, bigint, Array<boolean>];
export interface page {
  'remaining_count' : bigint,
  'comments' : Array<comment_output>,
}
export interface status {
  'time_created' : bigint,
  'is_empty' : boolean,
  'remaining_cycles' : bigint,
}
export interface toggle_metadata_param {
  'channel_id' : string,
  'label' : string,
  'comment_id' : string,
}
export type upsert_comment_param = comment_input;
export interface _SERVICE {
  'delete_comment' : ActorMethod<[delete_comment_param], string>,
  'get_thread' : ActorMethod<[get_thread_param], page>,
  'status' : ActorMethod<[], status>,
  'toggle_metadata' : ActorMethod<[toggle_metadata_param], boolean>,
  'upsert_comment' : ActorMethod<[upsert_comment_param], string>,
}

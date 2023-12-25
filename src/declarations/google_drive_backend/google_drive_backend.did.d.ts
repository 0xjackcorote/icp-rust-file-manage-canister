import type { Principal } from '@dfinity/principal';
import type { ActorMethod } from '@dfinity/agent';

export type Error = { 'CreateFail' : { 'msg' : string } } |
  { 'NotFound' : { 'msg' : string } } |
  { 'UpdateFail' : { 'msg' : string } };
export interface File {
  'id' : bigint,
  'updated_at' : [] | [bigint],
  'content' : string,
  'mime_type' : string,
  'file_name' : string,
  'folder_id' : bigint,
}
export interface FilePayload {
  'content' : string,
  'mime_type' : string,
  'file_name' : string,
  'folder_id' : bigint,
}
export interface Folder {
  'id' : bigint,
  'folder_name' : string,
  'updated_at' : [] | [bigint],
}
export interface FolderPayload { 'folder_name' : string }
export type Result = { 'Ok' : File } |
  { 'Err' : Error };
export type Result_1 = { 'Ok' : Folder } |
  { 'Err' : Error };
export type Result_2 = { 'Ok' : Array<File> } |
  { 'Err' : Error };
export type Result_3 = { 'Ok' : Array<Folder> } |
  { 'Err' : Error };
export interface _SERVICE {
  'create_file' : ActorMethod<[FilePayload], Result>,
  'create_folder' : ActorMethod<[FolderPayload], Result_1>,
  'delete_file' : ActorMethod<[bigint], Result>,
  'get_all_files' : ActorMethod<[], Result_2>,
  'get_all_files_by_folder_id' : ActorMethod<[bigint], Result_2>,
  'get_all_files_by_folder_name' : ActorMethod<[string], Result_2>,
  'get_all_folders' : ActorMethod<[], Result_3>,
  'get_file' : ActorMethod<[bigint], Result>,
  'get_folder' : ActorMethod<[bigint], Result_1>,
  'get_folder_by_name' : ActorMethod<[string], Result_1>,
  'update_file' : ActorMethod<[bigint, FilePayload], Result>,
  'update_file_name' : ActorMethod<[bigint, string], Result>,
  'update_folder' : ActorMethod<[bigint, FolderPayload], Result_1>,
}

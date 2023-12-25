export const idlFactory = ({ IDL }) => {
  const FilePayload = IDL.Record({
    'content' : IDL.Text,
    'mime_type' : IDL.Text,
    'file_name' : IDL.Text,
    'folder_id' : IDL.Nat64,
  });
  const File = IDL.Record({
    'id' : IDL.Nat64,
    'updated_at' : IDL.Opt(IDL.Nat64),
    'content' : IDL.Text,
    'mime_type' : IDL.Text,
    'file_name' : IDL.Text,
    'folder_id' : IDL.Nat64,
  });
  const Error = IDL.Variant({
    'CreateFail' : IDL.Record({ 'msg' : IDL.Text }),
    'NotFound' : IDL.Record({ 'msg' : IDL.Text }),
    'UpdateFail' : IDL.Record({ 'msg' : IDL.Text }),
  });
  const Result = IDL.Variant({ 'Ok' : File, 'Err' : Error });
  const FolderPayload = IDL.Record({ 'folder_name' : IDL.Text });
  const Folder = IDL.Record({
    'id' : IDL.Nat64,
    'folder_name' : IDL.Text,
    'updated_at' : IDL.Opt(IDL.Nat64),
  });
  const Result_1 = IDL.Variant({ 'Ok' : Folder, 'Err' : Error });
  const Result_2 = IDL.Variant({ 'Ok' : IDL.Vec(File), 'Err' : Error });
  const Result_3 = IDL.Variant({ 'Ok' : IDL.Vec(Folder), 'Err' : Error });
  return IDL.Service({
    'create_file' : IDL.Func([FilePayload], [Result], []),
    'create_folder' : IDL.Func([FolderPayload], [Result_1], []),
    'delete_file' : IDL.Func([IDL.Nat64], [Result], []),
    'get_all_files' : IDL.Func([], [Result_2], ['query']),
    'get_all_files_by_folder_id' : IDL.Func([IDL.Nat64], [Result_2], ['query']),
    'get_all_files_by_folder_name' : IDL.Func(
        [IDL.Text],
        [Result_2],
        ['query'],
      ),
    'get_all_folders' : IDL.Func([], [Result_3], ['query']),
    'get_file' : IDL.Func([IDL.Nat64], [Result], ['query']),
    'get_folder' : IDL.Func([IDL.Nat64], [Result_1], ['query']),
    'get_folder_by_name' : IDL.Func([IDL.Text], [Result_1], ['query']),
    'update_file' : IDL.Func([IDL.Nat64, FilePayload], [Result], []),
    'update_file_name' : IDL.Func([IDL.Nat64, IDL.Text], [Result], []),
    'update_folder' : IDL.Func([IDL.Nat64, FolderPayload], [Result_1], []),
  });
};
export const init = ({ IDL }) => { return []; };

export const idlFactory = ({ IDL }) => {
  const page = IDL.Rec();
  const delete_comment_param = IDL.Record({
    'channel_id' : IDL.Text,
    'comment_id' : IDL.Text,
  });
  const get_thread_param = IDL.Record({
    'channel_id' : IDL.Text,
    'cursor' : IDL.Opt(IDL.Text),
    'limit' : IDL.Nat8,
  });
  const metadata_output = IDL.Tuple(IDL.Text, IDL.Nat64, IDL.Bool);
  const comment_output = IDL.Record({
    'id' : IDL.Text,
    'content' : IDL.Text,
    'metadata' : IDL.Vec(metadata_output),
    'created_at' : IDL.Nat,
    'user_id' : IDL.Text,
    'replies' : page,
  });
  page.fill(
    IDL.Record({
      'remaining_count' : IDL.Nat,
      'comments' : IDL.Vec(comment_output),
    })
  );
  const toggle_metadata_param = IDL.Record({
    'channel_id' : IDL.Text,
    'label' : IDL.Text,
    'comment_id' : IDL.Text,
  });
  const comment_input = IDL.Record({
    'channel_id' : IDL.Text,
    'parent_id' : IDL.Opt(IDL.Text),
    'message' : IDL.Text,
    'comment_id' : IDL.Text,
  });
  const upsert_comment_param = comment_input;
  return IDL.Service({
    'delete_comment' : IDL.Func([delete_comment_param], [IDL.Text], []),
    'get_thread' : IDL.Func([get_thread_param], [page], ['query']),
    'toggle_metadata' : IDL.Func([toggle_metadata_param], [IDL.Bool], []),
    'upsert_comment' : IDL.Func([upsert_comment_param], [IDL.Text], []),
  });
};
export const init = ({ IDL }) => { return []; };

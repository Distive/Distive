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
  const comment_output = IDL.Record({
    'id' : IDL.Text,
    'content' : IDL.Text,
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
    'upsert_comment' : IDL.Func([upsert_comment_param], [IDL.Text], []),
  });
};
export const init = ({ IDL }) => { return []; };

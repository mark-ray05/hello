export const idlFactory = ({ IDL }) => {
  return IDL.Service({
    'get_submitted_names' : IDL.Func([], [IDL.Vec(IDL.Text)], ['query']),
    'greet' : IDL.Func([IDL.Text], [IDL.Text], []),
  });
};
export const init = ({ IDL }) => { return []; };

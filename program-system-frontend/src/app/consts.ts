import { HexString } from '@gear-js/api';

interface ContractSails {
  programId: HexString,
  idl: string
}

export const ACCOUNT_ID_LOCAL_STORAGE_KEY = 'account';

export const ADDRESS = {
  NODE: 'wss://testnet.vara.network', // import.meta.env.VITE_NODE_ADDRESS,
  BACK: import.meta.env.VITE_BACKEND_ADDRESS,
  GAME: import.meta.env.VITE_CONTRACT_ADDRESS as HexString,
};

export const ROUTES = {
  HOME: '/',
  EXAMPLES: '/examples',
  NOTFOUND: '*',
};

// To use the example code, enter the details of the account that will pay the vouchers, etc. (name and mnemonic)
export const sponsorName = "";
export const sponsorMnemonic = "";

export const CONTRACT_DATA: ContractSails = {
  programId: '0x9edc58323752ebc6aa65a29f5410b3e1b555a9e3dcbda2750653d6dd29321a4c',
  idl: `
  type QueryEvents = enum {
  ReceiverContractIdChanged,
  StringValueFromContract: str,
  NumValueFromContract: u64,
  VecStringValueFromContract: vec str,
  VecActorIdValueFromContract: vec actor_id,
  ErrorInReceiverContractResponse,
  ReceiverContractIdNotSpecified,
};

type TransmitterEvents = enum {
  ReceiverContractIdChanged,
  StringValueChanged: struct { new: str, old: str },
  NumValueChanged: struct { new: u64, old: u64 },
  StringValueAdded: struct { value_added: str },
  StringValueFromContract: str,
  NumValueFromContract: u64,
  VecStringValueFromContract: vec str,
  ReceiverContractIdNotSpecified,
  ErrorInReceiverContractResponse,
  IncorrectAnswerFromReceiverContract,
};

constructor {
  New : ();
  NewWithReceiverId : (receiver_id: actor_id);
};

service QueryService {
  query AllCallersFromReceiverContractState : () -> QueryEvents;
  query ContractOwner : () -> actor_id;
  query NumValueFromReceiverContractState : () -> QueryEvents;
  query ReceiverId : () -> opt actor_id;
  query StringValueFromReceiverContractState : () -> QueryEvents;
  query VecStringValueFromReceiverContractState : () -> QueryEvents;
};

service Transmitter {
  AddStringToReceiverContractVecString : (value: str) -> TransmitterEvents;
  ChangeReceiverContractNumValue : (new_val: u64) -> TransmitterEvents;
  ChangeReceiverContractStringValue : (new_val: str) -> TransmitterEvents;
  NumValueFromReceiverContract : () -> TransmitterEvents;
  SetReceiverContractId : (receiver_id: actor_id) -> TransmitterEvents;
  StringValueFromReceiverContract : () -> TransmitterEvents;
  VecStringFromReceiverContract : () -> TransmitterEvents;
};
  `
}
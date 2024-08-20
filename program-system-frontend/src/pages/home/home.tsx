import { useState, useEffect } from "react";
import { useSailsCalls } from "@/app/hooks";
import { useAccount, useAlert } from "@gear-js/react-hooks";
import { web3FromSource } from "@polkadot/extension-dapp";
import { Center, Button } from "@chakra-ui/react";
import './home.css';

export const Home = () => {
  const sails = useSailsCalls();
  const alert = useAlert();
  const { account } = useAccount();
  const [stateNumber, setStateNumber] = useState(0);
  const [stateString, setStateString] = useState('');
  const [stringVec, setStringVec] = useState<string[]>([]);
  const [inputNum, setInputNum] = useState(0);
  const [inputString, setInputString] = useState('test');
  const [inputVecString, setInputVecString] = useState('test');

  const sendMessage = async (method: string, payload: any) => {
    if (!sails) {
      console.error('Sails is not started!');
      return false;
    }

    if (!account)  {
      console.error('Cant get account!');
      return false;
    }

    const { signer } = await web3FromSource(account.meta.source);

    const response = sails.command(
      `Transmitter/${method}`,
      {
        userAddress: account.decodedAddress,
        signer
      },
      {
        callArguments: [ payload ],
        callbacks: {
          onLoad: () => alert.info('Loading message!'),
          onSuccess: () => alert.success('Message sent!'),
          onBlock: (blockHash) => alert.info(`ID: ${blockHash}`),
          onError: () => alert.error('Error while sending message')
        }
      }
    );

    return response;
  }

  const readState = async (method: string) => {
    if (!sails) {
      console.error('Sails is not started!');
      return false;
    }

    if (!account)  {
      console.error('Cant get account!');
      return false;
    }

    const response = sails.query(
      `QueryService/${method}`,
      {
        userId: account.decodedAddress,
      }
    );

    return response;
  }

  useEffect(() => {
    const func = async () => {
      const numState = await readState('NumValueFromReceiverContractState');
      const stringState = await readState('StringValueFromReceiverContractState');
      const vecStringState = await readState('VecStringValueFromReceiverContractState');

      const { vecStringValueFromContract } = vecStringState;
      const { textValueFromContract } = stringState;  
      const { numValueFromContract } = numState;
      
      setStateNumber(numValueFromContract);
      setStateString(textValueFromContract);
      setStringVec(vecStringValueFromContract);
    };

    const intervalId = setInterval(func, 500);

  
    return () => clearInterval(intervalId);
  }, [sails])
  

  return (
    <Center>
      <div className="examples-container">
          <div className="number-section">
            <div className="options-container">
              <div>
                <label htmlFor="num-field">(number) Change to: </label>
                <input 
                  type="number" 
                  name="num-field" 
                  id="num-field" 
                  className="example-input" 
                  defaultValue={inputNum}
                  onChange={(e) => {
                    if (e.target.value.trim() === '') {
                      alert.error('Cant ve empty!');
                      return;
                    }

                    const value = parseInt(e.target.value);

                    if (value < 0) {
                      alert.error('cant send negative numbers');
                      return;
                    }

                    setInputNum(value);
                  }}
                />
              </div>
              <Button
                onClick={async () => {
                  await sendMessage(
                    'ChangeReceiverContractNumValue',
                    inputNum
                  );
                }}
              >
                Send number
              </Button>
            </div>
            <p>Actual value: {stateNumber}</p>
          </div>


          <div className="string-section">
            <div className="options-container">
              <div>
                <label htmlFor="string-field">(string) Change to: </label>
                <input 
                  type="text" 
                  name="string-field" 
                  id="string-field" 
                  className="example-input" 
                  defaultValue={inputString}
                  onChange={(e) => {
                    const value = e.target.value.trim();

                    if (value === '') {
                      alert.error('Cant ve empty!');
                      return;
                    }
                    
                    setInputString(value);
                  }}
                />
              </div>
              <Button
                onClick={async () => {
                  await sendMessage(
                    'ChangeReceiverContractStringValue',
                    inputString
                  );
                }}
              >
                Send string
              </Button>
            </div>
            <p className="p-state-data">Actual value: {stateString}</p>
          </div>


          <div className="vec-section">
            <div className="options-container">
              <div>
                <label htmlFor="vec-str-field">(string) Add: </label>
                <input 
                  type="text" 
                  name="vec-str-field" 
                  id="vec-str-field" 
                  className="example-input" 
                  defaultValue={inputVecString}
                  onChange={(e) => {
                    const value = e.target.value.trim();

                    if (value === '') {
                      alert.error('Cant ve empty!');
                      return;
                    }
                    
                    setInputVecString(value);
                  }}
                />
              </div>
              <Button
                onClick={async () => {
                  await sendMessage(
                    'AddStringToReceiverContractVecString',
                    inputVecString
                  );
                }}
              >
                Add string
              </Button>
            </div>
            <p className="p-state-data">Actual values: {stringVec.join(', ')}</p>
          </div>
      </div>
    </Center>
  );
};
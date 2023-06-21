import fs from "fs";
import { createAdapter } from "iocane";
import pako from "pako";
const { gzip, ungzip } = pako;

const extractAndPrintBase64Data = (inputText) => {
  const lines = inputText.split('\n');

  for (const line of lines) {
    const base64Matches = line.match(/utf8\+base64:([^ ]+)/g);
    if (base64Matches) {
      let out = "";
      for (const base64Match of base64Matches) {
        const base64Data = base64Match.split('utf8+base64:')[1].trim();
        out += Buffer.from(base64Data, 'base64').toString('utf-8') + " ";
      }
      console.log(out);
    }
  }
};

const password = process.argv[3]
const vault = fs.readFileSync(process.argv[2], 'utf8')
const content = vault.replace(/b~>buttercup\/[ab]/, '')
const is_format_a = /b~>buttercup\/a/.test(vault)

createAdapter()
    .decrypt(content, password)
    .then(decryptedString => {
        if(is_format_a){
          extractAndPrintBase64Data(ungzip(decryptedString, { to: "string" })) 
        }else{
          console.log(JSON.stringify(JSON.parse(ungzip(Buffer.from(decryptedString, "base64"), { to: "string" })), null, 4))
        }
    });

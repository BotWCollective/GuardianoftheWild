 
let {popen} = require('../popen.node');
let fs = require('fs');
let message = "test message"
let user = "komali09"
let C = ["cpp", "c", "rs"];
let I = ["js", "py", "rb", "sh"];
let JIT = []
exports.__langs__ = C.concat(I).concat(JIT);
function RunCommand(command, message, user, self) {
  if(!command.trim().startsWith("!") || self) return
  message = `"` + message.replace(/\"/g, "\\\"") + `"`;
  command = command.substring(1,command.length);
  // console.log(command)
  let commandfiles = fs.readdirSync('./commands/cmd').map(x=>x.split('.'));
  if(!commandfiles.map(x=>x[0]).includes(command)) return
  let cmdfile = commandfiles.find(x=>x[0] == command)
  let CTR = "";
  switch(cmdfile[1]) {
   
    case "sh":
      CTR = `bash ./commands/cmd/${cmdfile[0]}.sh ${message} ${user}`;
      break;
  case "py":
      CTR = `python3 ./commands/cmd/${cmdfile[0]}.py ${message} ${user}`;
      break;
  case "rb":
      CTR = `ruby ./commands/cmd/${cmdfile[0]}.rb ${message} ${user}`;
      break;
  default:///lib*/ld-linux*
      CTR = `./commands/cmd/${cmdfile[0]} ${message} ${user}`;
      break;  
  }
  return popen(CTR)

}
function UploadCommand(command, fileext, code) {
  if(fs.readdirSync(`./commands/cmd/`).map(x=>x.split(".")[0]).includes(command)) popen(`rm ./commands/cmd/${command}.*`);
  
  fs.writeFileSync(`./commands/processing/${command}.${fileext}`, code);
  if(I.includes(fileext)) {
    fs.copyFileSync(`./commands/processing/${command}.${fileext}`, `./commands/cmd/${command}.${fileext}`);
  
    return 0;
  }
  if(C.includes(fileext)) {
    let CTR = "";
    switch(fileext) {
    case "cpp":
      CTR = `g++ ./commands/processing/${command}.${fileext} -o ./commands/cmd/${command}`;
      break;
    case "c":
      CTR = `gcc ./commands/processing/${command}.${fileext} -o ./commands/cmd/${command}`;
      break;
    case "rs":
      CTR = `rustc ./commands/processing/${command}.${fileext} -o ./commands/cmd/${command}`;
      break;
    
  }
  let output = popen(CTR);
  if(!fs.existsSync(`./commands/cmd/${command}`)) {
    // console.log(output)
    return {error: output, type: "Compiler Error"}; 
  }

  else return 0;
  }
  if(JIT.includes(fileext)) {
    return 0;
  }
popen("rm ./commands/processing/*")
}
// let pycode = `print(__import__('sys').argv)`
// UploadCommand('argv', 'py', pycode)
// let cppcode = `
// #include <iostream>
// int main(int argc, char**argv) {
//   std::cout << argv[1];
//   return 0;
// }`
// let rscode = `
// fn main() {
//   for a in std::env::args() { println!("{}", a); }
// }`
// UploadCommand('rstest', 'rs', rscode);
// console.log(RunCommand('rstest', message, user))
exports.UploadCommand = UploadCommand;
exports.RunCommand = RunCommand
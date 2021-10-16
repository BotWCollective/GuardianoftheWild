#!/bin/bash

# Small Bash Script I wrote to help updating the code onto my Raspberry Pi

clear
token=K0mAL1b0t2.T0k3nTh4tb3loNgst0KoM4l1.y0uM4yN0thav31t
urll=$(curl -s -H "Authorization: Bot $token" https://discord.com/api/v8/channels/012345678909876543/messages?limit=100 | node -e "text='';process.stdin.setEncoding('utf8'); process.stdin.on('readable', function () { var chunk = process.stdin.read(); if (chunk !== null) { text += chunk; } }); process.stdin.on('end', function () {
    text=JSON.parse(text);
    for(i in text) {
        if(text[i].attachments.length>0)if(text[i].attachments[0].filename.startsWith('gotw') && text[i].attachments[0].content_type=='application/zip') return process.stdout.write(text[i].attachments[0].url + '\n' + text[i].attachments[0].filename);
    }
});")
readarray -t url <<< "$urll"
if [ ! -f ${url[1]} ];  then 
  rm ${url[1]}
fi
echo Found URL: ${url[0]}
curl -R -J -O ${url[0]}
unzip -o ${url[1]} -d .

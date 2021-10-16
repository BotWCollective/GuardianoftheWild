//Komali, add more semicolons

let e = require("events");

let Timeouts = class {
    constructor() {
        this.obj = {}
    }
    set(n) {
        this.obj[n] = Date.now();
    }
    get(n) {
        return this.obj[n];
    }
    check(n, ms) {
        let d = Date.now();
        if (!this.obj[n] || d-this.obj[n]>=ms) {
            return true;
        }
        return false;
    }
}
class MessageCounter {
    constructor(value,tm,cb) {
        this.cb = cb
        this.v = value
        this.c = 0;
        this.tm = tm;
        this.t = 0;
    }
    inc() {
        if(Date.now() >= this.t) this.c++
        if(this.c>=this.v) {
            this.c=0;
            this.t = Date.now() + this.tm;
            this.cb();
        }       
    }
}

const SUBPREFIXES = [ //to prevent Subscriber emotes from appearing in !slots 
    "angryd18",
    "komali2",
    "kelski1",
    "bingsf",
    "rasenu",
    "makar0",
    "car0li1",
    "anigme",
    "zelkys",
    "cinnna1",
    "crazya27",
    "vladis17",
    "taoplu",
    "hylian56",
    "lieute32"
];



let twitchemotes = require("./emotes.json"); // taken from another project; the keys are the global emote names
const default_colors = ["#FF0000", "#0000FF", "#00FF00", "#B22222", "#FF7F50", "#9ACD32", "#FF4500", "#2E8B57", "#DAA520", "#D2691E", "#5F9EA0", "#1E90FF", "#FF69B4", "#8A2BE2", "#00FF7F"];
const discord = require("discord.js");
const tmi = require("tmi.js");
const fetch = require("node-fetch");
const fs = require("fs");

const bot = new e;
let t = new Timeouts();
const tmi_client = new tmi.client({
    identity: {
        username: "GuardianOfTheWild",
        password: require("./auth.json").token
    },
    channels: [
        "botwcollective"
    ]
});
tmi_client.connect();
let counter = new MessageCounter(200, 900000, ()=>tmi_client.say("botwcollective", "Having fun? Don't forget to clip to help out the highlight videos!"));
const djs_client = new discord.Client({ws: {Intents: discord.Intents.ALL}});
tmi_client.on("connected", () => {
    console.log("On! (twitch)");
});
djs_client.login(require("./auth.json").dtoken)
djs_client.on("ready", () => {
    console.log("On! (discord)");
    djs_client.user.setActivity("Breath of the Wild")
})




tmi_client.on("message",  (channel, user, _message, self) => {
    if(!self) counter.inc();
    let message = {
        content: _message,
        self: self,
        channel: channel,
        user: user,
        discord: false,
        twitch: true
    }
    bot.emit("message", message)
});
djs_client.on("message", message => {
    message.discord = true
    message.twitch = false;
    message.user = message.author
    message.user["display-name"] = message.author.username
    bot.emit("message", message)
})





bot.on("message", message => {
    let reply = (str) => {
        if(message.twitch) tmi_client.say(message.channel, str);
        if(message.discord) message.channel.send(str);
    }
    let user_lookup = (q, cb) => {
        if(message.twitch) {
            fetch("https://tmi.twitch.tv/group/user/" + message.channel.replace("#", "") + "/chatters").then(x=>x.json()).then(_users=>{
                let names = Object.values(Object.values(_users).flat().filter(x=>typeof x != "number" && Object.keys(x).length > 0)[0]).filter(x=>x.length>0).flat();       
                let unames = names.map(x=>x.toLowerCase());
                if(unames.includes(q.trim().toLowerCase()))return cb(names[unames.findIndex(x=>x==q.trim().toLowerCase())]);
                cb(null)
            })
        }
        if(message.discord) {
            message.guild.members.fetch().then(users=>{
                let names = users.map(x=>x.user.username)
                let unames = names.map(x=>x.toLowerCase())
                let ids = users.map(x=>x.user.id);
                if(unames.includes(q.trim().toLowerCase())) return cb(names[unames.findIndex(x=>x==q.trim().toLowerCase())]);
                if(ids.includes(q.replace(/[@<>\!]/g, "").trim())) return cb(names[ids.findIndex(x=>x==q.replace(/[@<>\!]/g, "").trim())]);
                cb(null)

            })  
        }
    }
    if(message.twitch) {
        if (message.content.trim().toLowerCase().includes("bigfollows") || message.content.trim().toLowerCase().includes("want to become famous") || message.content.trim().toLowerCase().includes("wanna become famous") || message.content.trim().toLowerCase().includes("wanna be famous") || message.content.trim().toLowerCase().includes("want to be famous")) {
            tmi_client.timeout(message.channel, user['display-name'], 1, "We don't like bots, do we?")
        }
        if (message.user && message.user.color == null) {
            var n = message.user['display-name'].charCodeAt(0) + message.user['display-name'].charCodeAt(message.user['display-name'].length - 1);
            message.user.color = default_colors[n % default_colors.length];
        }
    }

    let args = message.content.trim().split(" ");
    args[0] = args[0].toLowerCase();
    if (args[0] == "!discord") {
        reply("Join the discord server! discord.gg/hylian");
    }
    if (args[0] == "!hug") {
        if(!args[1]) {
            if (message.twitch) reply("/me hugs " + message.user["display-name"])
            if (message.discord) reply("*hugs " + message.user.toString() + "*")
            return
        }
        user_lookup(args[1],u=>{
            if(!u) u = args[1];
            if (message.twitch) reply("/me hugs " + u)
            if (message.discord) reply("*hugs " + u + "*")
        });
    }
    if (args[0] == "!slots") {
        let ms_timeout = 5000;
        let id;
        if(message.discord) id = message.user.id;
        if(message.twitch) id = message.user.username
        if(t.check(id, ms_timeout)) {
            t.set(id);
            var slotStr = "";
            var slotRepeat = args[1];
            if (isNaN(slotRepeat) || slotRepeat < 2) {
                slotRepeat = 3;
            }
            if (slotRepeat > 50) {
                slotRepeat = 50;
            }
            var str = ""
            for (i = 0; i < slotRepeat; i++) {
                function efilter(x){
                    for(s of SUBPREFIXES){ 
                     if(x.name.startsWith(s)) return false
                   }
                   return true
                 }
                var emotes = message.twitch ? Object.keys(twitchemotes) : message.guild.emojis.cache.array().filter(efilter).map(x=>x.toString())
                str = str + emotes[Math.floor(Math.random() * emotes.length)] + " "
            }  
            reply(str);
            reply("Please Play Again!");
        }
        else {
            reply(`You may not play for ${((t.get(id)+ms_timeout)-Date.now())/1000} seconds!`)
        }
    }
    if(args[0] == "!test") {
       //nothing here atm
    }
    if (args[0] == "!eval") {
        if (message.user.username != "komali09" && message.user.id != "327879060443234314") return;
        args.shift();
        try {
            let e = eval(args.join(" "));
        } catch (err) {
            reply(err.stack)
        }

    }
    if (args[0] == "!hi") {
        reply(`Hello, ${message.user["display-name"]}`);
    }
    
})

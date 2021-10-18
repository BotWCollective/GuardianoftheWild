// Komali, add more semicolons

// no >:( 


let e = require("events");
let fs = require("fs"); 

let CurrencySystem = class {
    constructor(type) {
        this.type=type;
        this.dat = {};
    }
    init() {
        this._loadjson();
    }
    _loadjson() {
        this.dat = JSON.parse(fs.readFileSync("./currency.json").toString());
    }
    _writejson() {
        fs.writeFileSync("./currency.json", JSON.stringify(this.dat));
    }
    get(user) {
        this._loadjson();
        if(!this.dat[user]) return 0;
        this._writejson();
        return this.dat[user].value;
    }
    getAll() {
        this._loadjson();
        let outObj = {
            twitch: [],
            discord: []
        };
        let _i;
        for(_i in Object.keys(this.dat)) {
            let i = Object.keys(this.dat)[_i];
            outObj[this.dat[i].sm].push((()=>{let _={};_[i]=this.dat[i].value;return _})()); // that was fun :D
        }
        this._writejson();
        return outObj;
    }
    change(user, amt, d) {
        if(!this.dat[user]) this.dat[user] = {value:0,sm:d};
        this.dat[user].value += amt;
        this.dat[user].sm = d; // Just in case
        this._writejson(); 
    }
}

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

const bot = new e;
let t = new Timeouts();
let cst = new Timeouts();
let cs = new CurrencySystem("<:rupee:899416200663158814>");
cs.init();
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
let counter = new MessageCounter(100, 900000, ()=>tmi_client.say("botwcollective", "Having fun? Don't forget to clip to help out the highlight videos!"));
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
    message.user.identification = message.user["display-name"];
    bot.emit("message", message)
});
djs_client.on("message", message => {
    message.discord = true
    message.twitch = false;
    message.user = message.author
    message.user["display-name"] = message.author.username;
    message.user.identification = message.user.id;
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
            tmi_client.timeout(message.channel, message.user['display-name'], 1, "We don't like bots, do we?")
        }
        if (message.user && message.user.color == null) {
            var n = message.user['display-name'].charCodeAt(0) + message.user['display-name'].charCodeAt(message.user['display-name'].length - 1);
            message.user.color = default_colors[n % default_colors.length];
        }
    }

    // =-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-
    if((message.twitch && message.channel == "#botwcollective" && !message.self) || (message.discord && !message.user.bot && message.guild && message.guild.id == "731993922414575737")) 
    {
        if(cst.check(message.user.identification, 60000)) {
            cst.set(message.user.identification);
            cs.change(message.user.identification, 1, message.discord ? "discord" : "twitch");
        }
    }
    // =-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-

    let args = message.content.trim().split(" ");
    args[0] = args[0].toLowerCase();
    if (args[0] == "!discord") {
        reply("Join the discord server! discord.gg/hylian");
    }
    if (args[0] == "!yt" || args[0] == "!youtube") {
        reply("Subscribe to our Youtube channel! https://www.youtube.com/channel/UC7C948AM_7cNIORd22Rr_SQ");
    }
    if (args[0] == "!twitter") {
        reply("Go follow our Twitter page! https://twitter.com/BotwCollective");
    }
    if (args[0] == "!bingothon") {
        reply("Go follow Bingothon! https://twitch.tv/bingothon");
    }
    if (args[0] == "!bal" || args[0] == "!balance") {
        reply(`You have ${cs.get(message.user.identification)} ${cs.type}`)
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
        let id = message.user.identification;
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
    if(args[0] == "!lb" || args[0] == "!leaderboard") {
        if (message.twitch) return;
        let lbsm = args[1] ? args[1] : "all";
        if (args[1] && (args[1] == "discord" || args[1] != "twitch")) {
            return reply("Argument 2 must be \`discord\`, \`twitch\` or undefined");
        }

        let lb = cs.getAll();
        if(lbsm != "all") lb = lb[lbsm];
        else {
            let arr = [];
            lb.discord.forEach(x=>{
                let _o = {};
                _o["<@"+Object.keys(x)[0]+">"] = Object.values(x)[0]
                arr.push(_o);
            })
            lb.twitch.forEach(x=>{
                let _o = {};
                _o["`"+Object.keys(x)[0]+"`"] = Object.values(x)[0]
                arr.push(_o);
            })
            lb = arr;
            
        }
        lb = lb.sort((a,b)=>(Object.values(b)[0] - Object.values(a)[0]));
        let embed = new discord.MessageEmbed();
        embed.setTitle((lbsm == "discord" ? "Discord" : lbsm == "twitch" ? "Twitch" : "Full") + " Leaderboard");
        embed.setColor("#1a7ce2");
        embed.setFooter("You can do !lb <discord|twitch> for a more specific leaderboard!");
        if(lbsm == "discord") {
            embed.setDescription(
                lb.map(x=>`<@${Object.keys(x)[0]}>: ${Object.values(x)[0]} ${cs.type}`)
            );
        }
        else if(lbsm == "twitch") {
            embed.setDescription(
                lb.map(x=>`\`${Object.keys(x)[0]}\`: ${Object.values(x)[0]} ${cs.type}`)
            );
        }
        else {
            embed.setDescription(
                lb.map(x=>`${Object.keys(x)[0]}: ${Object.values(x)[0]} ${cs.type}`)
            );
        }
        reply(embed);
        
        
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

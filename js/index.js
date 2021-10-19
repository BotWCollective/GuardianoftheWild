// Komali, add more semicolons

// no >:( 


let e = require("events");
let fs = require("fs"); 

let CurrencySystem = class {
    constructor() {
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
    getRupee(n) {
        let green = "<:rupee:899771488792608789>";
        let blue = "<:rupee5:899770964529807430>";
        let red = "<:rupee20:899770964454289468>";
        let purple = "<:rupee50:899770964475265024>";
        let silver = "<:rupee100:899770964483637328>";
        let gold = "<:rupee300:899770964630441995>";
        let r;
        switch(true) { // bad syntax is a result of me forgetting how to use switch() :I
            case n < 5:
                r = green;
                break;
            case n < 20: 
                r =  blue;
                break;
            case n < 50:
                r =  red;
                break;
            case n < 100: 
                r =  purple;
                break;
            case n < 300:
                r =  silver
                break;
            case n >= 300:
                r = gold;
                break;
        }
        return r;
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
let gst = new Timeouts();
let cs = new CurrencySystem();
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
    let args = message.content.trim().split(" ");
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
    if(args[0] == "!gamble") {
        if(gst.check(message.user.identification, 600000)) {
            let current = cs.get(message.user.identification);
            if(current <= 0) return reply("Get some " + cs.getRupee(1) + " first!")
            if(!args[1]) return reply("A second argument is required!");
            if(args[1].toLowerCase() == "all") args[1] = current;
            if(isNaN(args[1])) return reply("Argument 1 is not a number!");
            
            args[1] = parseInt(args[1]);
            if(current < args[1]) return reply("You do not have " + args[1] + cs.getRupee(args[1]));
            if(args[1] < 1) return reply(`No, you can't bet ${args[1] + cs.getRupee(args[1])}. Nice Try.`)
            if(Math.random()>0.5) {
                cst.set(message.user.identification);
                cs.change(message.user.identification, args[1]*-1, message.discord ? "discord" : "twitch") 
                reply(`You lost ${args[1]}${cs.getRupee(args[1])}. You now have ${cs.get(message.user.identification)}${cs.getRupee(cs.get(message.user.identification))}`);
            }
            else {
                cst.set(message.user.identification);
                cs.change(message.user.identification, args[1], message.discord ? "discord" : "twitch") 
                reply(`You won ${args[1]}${cs.getRupee(args[1])}. You now have ${cs.get(message.user.identification)}${cs.getRupee(cs.get(message.user.identification))}`);
            }
            return gst.set(message.user.identification);

        }
        else {
            let _t = ((gst.get(message.user.identification)+600000)-Date.now());
            return reply(`You may not play for ${new Date(_t).toISOString().substr(15, 8)}!`)
        }
    }

    // =-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-
    if((message.twitch && message.channel == "#botwcollective" && !message.self) || (message.discord && !message.user.bot && message.guild && message.guild.id == "731993922414575737")) 
    {
        if(cst.check(message.user.identification, 60000)) {
            cst.set(message.user.identification);
            let num = 1;
            if(cs.get(message.user.identification) < 0) num = cs.get(message.user.identification) * -1; 
            cs.change(message.user.identification, num, message.discord ? "discord" : "twitch");
        }
    }
    // =-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-

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
        let v = cs.get(message.user.identification);
        reply(`You have ${v.toString().replace(/\B(?=(\d{3})+(?!\d))/g, ",")}${message.discord ? cs.getRupee(v) : " Rupees"}!`)
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
                   return true;
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
        let lbsm = "all";
        let pgct = 1;
        if(args.length >= 1) {
            for(i in args){
                if(i==0) continue;
                if(!isNaN(args[i])) pgct = parseInt(args[i]);
                else if(args[i] == "discord") lbsm = "discord";
                else if(args[i] == "twitch") lbsm = "twitch";
                else if(args[i] != "all") return reply(`Invalid Argument at position ${i- -1}: \`${args[1]}\``);
            }
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
        let pages = [];
        for (let i = 0, j = lb.length; i < j; i += 15) {
            pages.push(lb.slice(i, i + 15));
        }
        if(pgct > pages.length || pgct <= 0) return reply("Invalid page index: `" + pgct + "`");
        pgct--;
        let embed = new discord.MessageEmbed();
        embed.setTitle(`${lbsm == "discord" ? "Discord" : lbsm == "twitch" ? "Twitch" : "Full"} Leaderboard (Page ${pgct+1}/${pages.length})`);
        embed.setColor("#1a7ce2");
        embed.setFooter("You can do !lb <discord|twitch> for a more specific leaderboard!");
        if(lbsm == "discord") {
            embed.setDescription(
                pages[pgct].map(x=>`<@${Object.keys(x)[0]}>: ${Object.values(x)[0].toString().replace(/\B(?=(\d{3})+(?!\d))/g, ",")}${cs.getRupee(Object.values(x)[0])}`)
            );
        }
        else if(lbsm == "twitch") {
            embed.setDescription(
                pages[pgct].map(x=>`\`${Object.keys(x)[0]}\`: ${Object.values(x)[0].toString().replace(/\B(?=(\d{3})+(?!\d))/g, ",")}${cs.getRupee(Object.values(x)[0])}`)
            );
        }
        else {
            embed.setDescription(
                pages[pgct].map(x=>`${Object.keys(x)[0]}: ${Object.values(x)[0].toString().replace(/\B(?=(\d{3})+(?!\d))/g, ",")}${cs.getRupee(Object.values(x)[0])}`)
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

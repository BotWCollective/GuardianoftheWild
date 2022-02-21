// Komali, add more semicolons

// no >:( 


let e = require("events");
let fs = require("fs"); 
let WS = require("ws");
const auth = require("./auth.json");

const UsernameExceptions = { //Twitch/Discord syncing for those whos names do not match
    "354602994911674368": "taoplusplus",
    "327879060443234314": "komali09",
    "312830913912242176": "powergaymerkai",
    "706242928363700294": "takosensei101",
    "448790275372875789": "epicsgeiler",
    "673550715137818635": "z2w_02",
    "161576331505696768": "mrpancaketurtle",
    "435878781891248138": "sloop28_",
    "488708336624074753": "coensi",
    "536179895047290881": "arcturus_botw",
    //People who don't commentate but are still active
    "659901334266576926": "cinnnamon_",
    "772287021107511307": "p3ngu1nc4t",
    "247204362273816576": "the_bromie",
    "298527618871984138": "surprisedpika_"

}

let CurrencySystem = class {
    constructor() {
        this.dat = {};
    }
    init() {
        this._loadjson();
        let _i;
        for(_i in Object.keys(this.dat)) {
            let i = Object.keys(this.dat)[_i];
            if(isNaN(this.dat[i].value)) this.dat[i].value = 0; // Useful for times like when savage realized you could do !gamble Infinity
        }
        this._writejson();
    }
    _loadjson() {
        this.dat = JSON.parse(fs.readFileSync("./currency.json").toString());
        for(let i in Object.keys(this.dat)) {
            if(!this.dat[Object.keys(this.dat)[i]].sm) {
                if(isNaN(Object.keys(this.dat)[i])) this.dat[Object.keys(this.dat)[i]].sm = "twitch";
                else this.dat[Object.keys(this.dat)[i]].sm = "discord"
            }
        }
        this._writejson();
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
            console.log(this.dat[i].sm)
            if(outObj[this.dat[i].sm]) outObj[this.dat[i].sm].push((()=>{let _={};_[i]=this.dat[i].value;return _})()); // that was fun :D
        }
        this._writejson();
        return outObj;
    }
    getRupee(n, d=false) {
        if(d) return " Rupee" + n == 1 ? "" : "s";
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
let MessageCounter = class {
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
let BingoBongo = class extends e {
    constructor() {
        super(); // Since when can you just call the function :0
        this.room = "";
        this.alert = new e;
        this.ws = null;//new WS;
        this.pwd = "";
    }
    set(t,p,b="", c) {
        this.room = t;
        this.pwd = p;
        this.update(b, s=>{
            c(s);
        })
    }
    delete() {
        if(this.ws) this.ws.close();
        this.ws = null;
    }
    update(b,c) {
        if(this.ws) this.ws.close();
        let URL = 'https://bingosync.bingothon.com'; 
        let WSS = "wss://bingosock.bingothon.com/broadcast"
        if(b=="default") {
            URL = 'https://bingosync.com';
            WSS = "wss://sockets.bingosync.com/broadcast"   
        }
        fetch(URL + "/api/join-room", {
            method: "POST",
            headers: {
                'Accept': 'application/json',
                'Content-Type': 'application/json'
            },
            body: JSON.stringify({
                room: this.room,
                nickname: "GuardianoftheWild",
                password: this.pwd
            })
        }).then(x=>x.json()).then(res=>{
            if(res.__all__) {
                for(let i in res.__all__) if(Object.keys(res.__all__[i]).includes("Incorrect Password")) return c("Incorrect Password");
            }
            if(!res.socket_key) return c("No socket key obtained, try checking the command again");
            let key = res.socket_key;
            this.ws = new WS(WSS);
            this.ws.on("open", _=>{
                this.ws.send(`{"socket_key": "${key}"}`);
                c(`Connected to \`${WSS}\` (Room \`${this.room}\`)`);
            })
            this.ws.on("message", message => {
                message = JSON.parse(message.toString());
                if(message.type == "goal") {
                    this.emit("msg", `${message.player.name} has ${message.remove?"un":""}marked "${message.square.name}"`);
                }
                if(message.type == "chat") {
                    this.emit("msg", `${message.player.name}: "${message.text}"`);
                }
                if(message.type == "connection") {
                    this.emit("msg", `${message.player.name} has joined!`);
                }
            })
        }).catch(x=>{
            console.log(x);
            c("Something went wrong....");
        })
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
    "lieute32",
    "botwco",
    "rosie0"
];



let twitchemotes = require("./emotes.json"); // taken from another project; the keys are the global emote names
const default_colors = ["#FF0000", "#0000FF", "#00FF00", "#B22222", "#FF7F50", "#9ACD32", "#FF4500", "#2E8B57", "#DAA520", "#D2691E", "#5F9EA0", "#1E90FF", "#FF69B4", "#8A2BE2", "#00FF7F"];
const discord = require("discord.js");
const tmi = require("tmi.js");
const c = require("fetch-cookie");
const fetch = require("fetch-cookie/node-fetch")(require("node-fetch"));

const bot = new e;
const Bingo = new BingoBongo;
let t = new Timeouts();
let cst = new Timeouts();
let gst = new Timeouts();
let cs = new CurrencySystem();
cs.init();
const tmi_client = new tmi.client({
    identity: {
        username: "GuardianOfTheWild",
        password: auth.token
    },
    channels: [
        "botwcollective"
    ]
});
tmi_client.connect();
let counter = new MessageCounter(100, 300000, ()=>tmi_client.say("botwcollective", "Having fun? Don't forget to clip to help out the highlight videos!"));
const djs_client = new discord.Client({ws: {Intents: discord.Intents.ALL}});
tmi_client.on("connected", () => {
    console.log("On! (twitch)");
});
djs_client.login(auth.dtoken)
djs_client.on("ready", () => {
    console.log("On! (discord)");
    djs_client.user.setActivity("Breath of the Wild")
})

// BINGO SETUP ====================================


Bingo.on("msg", message =>{
    tmi_client.say("botwcollective", "[Bingosync] " + message);
})


// END BINGO SETUP ================================


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
    message.user.mod = message.member?message.member.roles.cache.has("855458975989891122") || message.member.roles.cache.has("793207014690914325"):!1; //Specific to this server
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
    if (message.discord && message.content.includes("<@!897875834805817394>")) {
        fetch(`https://discord.com/api/v8/channels/${message.channel.id}/messages`, {
            method: "POST",
            headers: {
                "Authorization": `Bot ${auth.dtoken}`,
                "Content-Type": "application/json"
            },
            body: JSON.stringify({
                "content": "_\\*Fires a laser in your general direction\\*_",
                "message_reference": {
                    "message_id": message.id.toString(),
                    "guild_id": message.guild.id
                }
            })
      })
    }
    if(args[0] == "!bingo") {
        let b = "";
        if(!message.user.mod) return reply("You must be a moderator to run this!");
        if(args[1] == "remove" || args[1] == "rm") {
            Bingo.delete();
            return reply("Disconnected WebSocket!")
        }
        if(args[3] == "default") {
            b = "default";
            args.pop();
        }
        
        if(args.length != 3) return reply("Syntax: `!bingo {room} {password} {default (optional)}`");
        Bingo.set(args[1].trim(), args[2].trim(), b, out => {
            return reply(out);
        })

    }
    if(args[0] == "!give") {
        let current = cs.get(message.user.identification);
        if(current <= 0) return reply("Get some " + cs.getRupee(1,message.twitch) + " first!")
        if(args.length!=3) return reply("Syntax: `!give {user} {amount}`");
        if(args[2].toLowerCase() == "all") args[2] = current;
        if(isNaN(args[2])) return reply("Argument 1 is not a number!");
        
        args[2] = parseInt(args[2]);
        if(current < args[2]) return reply("You do not have " + args[2] + cs.getRupee(args[2],message.twitch));
        if(args[2] < 1 || isNaN(args[2])) return reply(`That's not how giving works!`);
        user_lookup(args[1], u=>{
            if(!u) return reply("This user cannot be found!");
            if(message.discord) {
                message.guild.members.fetch().then(users=>{
                    for(let user of users) {
                        if(user[1].user.username == u) {
                            cs.change(message.user.identification, args[2]*-1, "discord");
                            cs.change(user[1].user.id, args[2], "discord");
                            return reply(`Gave ${args[2]}${cs.getRupee(args[2],message.twitch)} rupees to <@${user[1].user.id}>`);
                        }
                    }
                })
            }
            if(message.twitch) {
                cs.change(message.user.identification, args[2]*-1, "twitch");
                cs.change(u, args[2], "twitch");
                return reply(`Gave ${args[2]}${cs.getRupee(args[2],message.twitch)} rupees to <@${u}>`);
            }
        })
    }
    if(args[0] == "!gamble") {
        if(gst.check(message.user.identification, 600000)) {
            let current = cs.get(message.user.identification);
            if(current <= 0) return reply("Get some " + cs.getRupee(1,message.twitch) + " first!")
            if(!args[1]) return reply("A second argument is required!");
            if(args[1].toLowerCase() == "all") args[1] = current;
            if(isNaN(args[1])) return reply("Argument 1 is not a number!");
            
            args[1] = parseInt(args[1]);
            if(current < args[1]) return reply("You do not have " + args[1] + cs.getRupee(args[1],message.twitch));
            if(args[1] < 1 || isNaN(args[1])) return reply(`No, you can't bet ${args[1] + cs.getRupee(args[1],message.twitch)}. Nice Try.`);

            if(new Date%2<1) {
                gst.set(message.user.identification);
                cs.change(message.user.identification, args[1]*-1, message.discord ? "discord" : "twitch"); 
                reply(`You lost ${args[1]}${cs.getRupee(args[1],message.twitch)}. You now have ${cs.get(message.user.identification)}${cs.getRupee(cs.get(message.user.identification),message.twitch)}`);
            }
            else {
                gst.set(message.user.identification);
                cs.change(message.user.identification, args[1], message.discord ? "discord" : "twitch") 
                reply(`You won ${args[1]}${cs.getRupee(args[1],message.twitch)}. You now have ${cs.get(message.user.identification)}${cs.getRupee(cs.get(message.user.identification),message.twitch)}`);
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
    if (args[0] == "!music" || args[0] == "!musiccredits") {
        reply("The music on our stream was made by direct.me/nintnt (an Ambient Guitar/Synth VGM cover artist) who kindly allowed us music use for these events! Follow and support em or else ٩(＾◡＾)۶");
    }
    if(args[0] == "!hbc") {
        reply("Information for the Hyrule Bike Challenge can be found at https://komali.dev/hbc and https://speedrun.com/botw_extension/Hyrule_Bike_Challenge")
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
        reply(`You have ${v.toString().replace(/\B(?=(\d{3})+(?!\d))/g, ",")}${cs.getRupee(v,message.twitch)}!`)
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
    if(args[0] == "!so") {
        if(!args[1]) return reply("You need to specify someone!");
        if(message.discord) {
            user_lookup(args[1], u=>{
                if(!u) return reply("That user does not exist!");
                return reply(`Thanks for the support! ${u} is a friend of the botwco, sacrifice a new tab and follow em!`);
            })
        }
        if(message.twitch) {
            fetch(`https://id.twitch.tv/oauth2/token?client_id=${auth.clid}&client_secret=${auth.cls}&grant_type=client_credentials`, {method: 'POST'})
            .then(res => res.json())
            .then(res => {
                var token = res.access_token;
                fetch("https://api.twitch.tv/helix/users?login="+args[1], {
                    headers: {
                        'Accept': "application/vnd.twitchtv.v5 + json",
                        'Client-ID': auth.clid,
                        'Authorization': 'Bearer ' + token,
                    }
                }).then(x=>x.json()).then(x=>{
                    if(x.data.length < 1) {
                        return reply("That user does not exist!");
                    }
                    let uuid = x.data[0].id;
                    fetch("https://api.twitch.tv/kraken/channels/"+uuid, {
                        headers: {
                            'Accept': "application/vnd.twitchtv.v5 + json",
                            'Client-ID': auth.clid,
                            'Authorization': 'Bearer ' + token,
                        }
                    }).then(x=>x.json()).then(x=>{
                        reply(`Thanks for the support! ${x.display_name} is a friend of the botwco, sacrifice a new tab and follow em at twitch.tv/${x.display_name.toLowerCase()}! They were last live streaming ${x.game}`)
                    })
                })
            })
        }
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


    if(["!commentators","!comms","!coms"].includes(args[0])) {
        let channel = args.includes("-fos")?"731993923001516075":"901620873742659585";
        let M = djs_client.channels.cache.get(channel).members.array();
        M = M.map(x=>`${x.user.username} (${message.discord?"<https://":''}twitch.tv/${
            Object.keys(UsernameExceptions).includes(x.user.id) ? UsernameExceptions[x.user.id] : x.user.username.toLowerCase().replace(/ /g, "_")
        }${message.discord?">":''})`).filter(n => !["31pnmr.alt", "bc_bot"].includes(n.split(".tv/")[1].split(message.discord?">":")")[0])); //Me being lazy and forgetting about this
        if(M.length < 1) return reply("There are no commentators currently!");
        else return reply("Go support our commentators! " + M.join(" | "))
    }
    
})

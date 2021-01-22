pub const DEMO_SCHEMA: &str = r#"{"id": "string","message_id": "u64","user_id": "i64", "guild_id": "i64", "channel_id": "i64", "timestamp": "date", "content": "text"}"#;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DemoMessage {
    pub id: String,
    pub message_id: i64,
    pub guild_id: i64,
    pub channel_id: i64,
    pub user_id: i64,
    pub timestamp: String,
    pub content: String,
}

pub const DEMO_DATA: &str = r#"[
{
"id" : "101",
"message_id" : 1,
"guild_id" : 139260719811133440,
"channel_id" : 139260947033358336,
"user_id" : 139259597750927360,
"timestamp" : "2020-09-02T14:14:44.000Z",
"content" : "In 1989 an 8.2 earthquake almost flattened America, killing over 30,000 people in less than four minutes. In the midst of utter devastation and chaos, a father left his wife safely at home and rushed to the school where his son was supposed to be, only to discover that the building was as flat as a pancake."
},
{
"id" : "102",
"message_id" : 2,
"guild_id" : 139260719811133440,
"channel_id" : 139260947033358336,
"user_id" : 139259496311685120,
"timestamp" : "2020-09-02T14:14:44.000Z",
"content" : "1989年，一次8.2级的地震几乎铲平美国，在短短不到4分钟的时间里，夺去了3万多人的生命!在彻底的破坏与混乱之中，有位父亲将他的妻子在家里安顿好后，跑到他儿子就读的学校，而触目所见，却是被夷为平地的校园。"
},
{
"id" : "103",
"message_id" : 3,
"guild_id" : 139260719811133440,
"channel_id" : 139260947033358336,
"user_id" : 139259665077895168,
"timestamp" : "2020-09-02T14:14:44.000Z",
"content" : "After the unforgettably initial shock, he remembered the promise he had made to his son: No matter what, I’ll always be there for you! And tears began to fill his eyes. As he looked at the pile of ruins that once was the school, it looked hopeless, but he kept remembering his commitment to his son."
},
{
"id" : "104",
"message_id" : 4,
"guild_id" : 139260719811133440,
"channel_id" : 139260947033358336,
"user_id" : 139259673680412672,
"timestamp" : "2020-09-02T14:14:44.000Z",
"content" : "看到这令人伤心的一幕，他想起了曾经对儿子所作的承诺：不论发生什么事，我都会在你身边。至此，父亲热泪满眶。目睹曾经的学校成为了一堆瓦砾，真叫人绝望。但父亲的脑中仍然牢记着他对儿子的诺言。"
},
{
"id" : "105",
"message_id" : 5,
"guild_id" : 139260719811133440,
"channel_id" : 139260947033358336,
"user_id" : 139259756622774272,
"timestamp" : "2020-09-02T14:14:44.000Z",
"content" : "He began to direct his attention towards where he walked his son to class at school each morning. Remembering his son s classroom would be in the back right corner of the building; he rushed there and started digging through the ruins."
},
{
"id" : "106",
"message_id" : 6,
"guild_id" : 139260719811133440,
"channel_id" : 139260947033358336,
"user_id" : 139259667846135808,
"timestamp" : "2020-09-02T14:14:44.000Z",
"content" : "他开始努力回忆每天早上送儿子上学的必经之路，终于记起儿子的教室应该就在那幢建筑物后面，位于右边的角落里，他跑到那儿，开始在碎石砾中挖掘，搜寻儿子的下落。"
},
{
"id" : "107",
"message_id" : 7,
"guild_id" : 139260719811133440,
"channel_id" : 139260947033358336,
"user_id" : 139259706517618688,
"timestamp" : "2020-09-02T14:14:44.000Z",
"content" : "As he was digging, other helpless parents arrived, clutching their hearts, saying: My son! My daughter! Other well meaning parents tried to pull him off what was left of the school, saying: It s too late! They’re all dead! You can’t help! Go home! Come on, face reality, there s nothing you can do!"
},
{
"id" : "108",
"message_id" : 8,
"guild_id" : 139260719811133440,
"channel_id" : 139260947033358336,
"user_id" : 139259604717666304,
"timestamp" : "2020-09-02T14:14:44.000Z",
"content" : "当这位父亲正在挖掘时，其他束手无策的学生家长赶到现场，揪心地叫着：我的儿子呀! 我的女儿呀!一些好意的家长试图把这位父亲劝离现场，告诉他一切都太迟了!他们全死了!这样做没用的，回去吧，这样做只会使事情更糟。"
},
{
"id" : "109",
"message_id" : 9,
"guild_id" : 139260719811133440,
"channel_id" : 139260947033358336,
"user_id" : 139259708363112448,
"timestamp" : "2020-09-02T14:14:44.000Z",
"content" : "To each parent he responded with one line: Are you going to help me now? And then he continued to dig for his son, stone by stone. The fire chief showed up and tried to pull him off the school s ruins saying, Fires are breaking out, explosions are happening everywhere. You’re in danger. We’ll take care of it. Go home. To which this loving, caring American father asked, Are you going to help me now?"
},
{
"id" : "110",
"message_id" : 10,
"guild_id" : 139260719811133440,
"channel_id" : 139260947033358336,
"user_id" : 139259551336759296,
"timestamp" : "2020-09-02T14:14:44.000Z",
"content" : "面对种种劝告，这位父亲的回答只有一句话：你们愿意帮我吗?然后继续进行挖掘工作，在废墟中寻找他的儿子。消防队长出现了，他也试图把这位父亲劝走，对他说：火灾频现，四处都在发生爆炸，你在这里太危险了，这边的事我们会处理，你回家吧!对此，这位慈爱、关切的父亲仍然回答：你们要帮我吗?"
},
{
"id" : "111",
"message_id" : 11,
"guild_id" : 139260719811133440,
"channel_id" : 139260947033358336,
"user_id" : 139259487176491008,
"timestamp" : "2020-09-02T14:14:44.000Z",
"content" : "The police came and said, You’re angry, anxious and it s over. You’re endangering others. Go home. We’ll handle it! To which he replied, Are you going to help me now? No one helped.Courageously he went on alone because he needed to know for himself: Is my boy alive or is he dead? He dug for eight hours...12 hours...24 hours...36 hours...then, in the 38th hour, he pulled back a large stone and heard his son s voice. He screamed his son s name, ARMAND! He heard back, Dad!?! It s me, Dad! I told the other kids not to worry. I told them that if you were alive, you d save me and when you saved me, they d be saved. You promised, No matter what happens, I’ll always be there for you! You did it, Dad! What s going on in there? How is it? the father asked."
},
{
"id" : "112",
"message_id" : 12,
"guild_id" : 139260719811133440,
"channel_id" : 139260947033358336,
"user_id" : 139259472064413696,
"timestamp" : "2020-09-02T14:14:44.000Z",
"content" : "警察赶到现场，对他说：你现在又气又急，该结束了，你在危及他人，回家吧!我们会处理一切的。这位父亲依旧回答：你们愿意帮我吗? 然而，人们无动于衷。为了弄清楚儿子是死是活，这位父亲独自一人鼓起勇气，继续进行他的工作。他挖掘了8小时，--12小时，24小时，36小时--38小时后，父亲推开了一块巨大的石头，听到了儿子的声音。父亲尖叫着：阿曼德!儿子的回音听到了：爸爸吗?是我，爸，我告诉其他的小朋友不要着急。我告诉他们如果你活着，你会来救我的。如果我获救了，他们也就获救了。你答应过我， 不论发生什么，我永远都会在你的身边， 你做到了，爸!你那里的情况怎样?父亲问。"
},
{
"id" : "113",
"message_id" : 13,
"guild_id" : 139260719811133440,
"channel_id" : 139260947033358336,
"user_id" : 139259716604919808,
"timestamp" : "2020-09-02T14:14:44.000Z",
"content" : "There are 14 of us left out of 33, Dad. We’re scared, hungry, thirsty and thankful you re here. When the building collapsed, it made a triangle, and it saved us."
},
{
"id" : "114",
"message_id" : 14,
"guild_id" : 139260719811133440,
"channel_id" : 139260947033358336,
"user_id" : 139259595204984832,
"timestamp" : "2020-09-02T14:14:44.000Z",
"content" : "我们有33个，只有14个活着。爸，我们好害怕，又渴又饿，谢天谢地，你在这儿。教室倒塌时，刚好形成一个三角形的洞，救了我们。"
},
{
"id" : "115",
"message_id" : 15,
"guild_id" : 139260719811133440,
"channel_id" : 139260947033358336,
"user_id" : 139259764508065792,
"timestamp" : "2020-09-02T14:14:44.000Z",
"content" : "The story goes that some time ago, a man punished his 3-year-old daughter for wasting a roll of gold wrapping paper. Money was tight and he became infuriated when the child tried to decorate a box to put under the Christmas tree. Nevertheless, the little girl brought the gift to her father the next morning and said, This is for you, Daddy."
},
{
"id" : "116",
"message_id" : 16,
"guild_id" : 139260719811133440,
"channel_id" : 139260947033358336,
"user_id" : 139259618240102400,
"timestamp" : "2020-09-02T14:14:44.000Z",
"content" : "有这样一个故事，爸爸因为三岁的女儿浪费了一卷金色的包装纸而惩罚了她。家里很缺钱，当孩子想要用包装纸装饰一个挂在圣诞树上的盒子时，爸爸生气了。然而，第二天早上小女孩把盒子作为礼物送给了爸爸，这是给你的，爸爸。"
},
{
"id" : "117",
"message_id" : 17,
"guild_id" : 139260719811133440,
"channel_id" : 139260947033358336,
"user_id" : 139259543283695616,
"timestamp" : "2020-09-02T14:14:44.000Z",
"content" : "The man was embarrassed by his earlier overreaction, but his anger flared again when he found out the box was empty. He yelled at her, stating, Don't you know, when you give someone a present, there is supposed to be something inside? The little girl looked up at him with tears in her eyes and cried, Oh, Daddy, it's not empty at all. I blew kisses into the box. They're all for you, Daddy."
},
{
"id" : "118",
"message_id" : 18,
"guild_id" : 139260719811133440,
"channel_id" : 139260947033358336,
"user_id" : 139259540557398016,
"timestamp" : "2020-09-02T14:14:44.000Z",
"content" : "女儿的这个行为让爸爸感到尴尬。但是当他发现盒子是空的时候，他的怒火再一次燃烧了。他对女儿喊道，难道你不知道给别人礼物的时候，里面应该放有东西吗?多女孩抬头看着父亲，眼里含着泪水，爸爸，盒子不是空的。我把吻放在了盒子里，都是给你的，爸爸。"
},
{
"id" : "119",
"message_id" : 19,
"guild_id" : 139260719811133440,
"channel_id" : 139260947033358336,
"user_id" : 139259771231535104,
"timestamp" : "2020-09-02T14:14:44.000Z",
"content" : "The father was crushed. He put his arms around his little girl, and he begged for her forgiveness. Only a short time later, an accident took the life of the child. It is also told that her father kept that gold box by his bed for many years and, whenever he was discouraged, he would take out an imaginary kiss and remember the love of the child who had put it there."
},
{
"id" : "120",
"message_id" : 20,
"guild_id" : 139260719811133440,
"channel_id" : 139260947033358336,
"user_id" : 139259770703052800,
"timestamp" : "2020-09-02T14:14:44.000Z",
"content" : "爸爸感动极了，他搂住女儿，恳请她的原谅。之后不久，一场事故夺走了小女孩的生命。据说，父亲便将那个小金盒子放在床头，一直陪伴着他的余生。无论何时他感到气馁或者遇到难办的事情，他就会打开礼盒，取出一个假想的吻，记起漂亮女儿给予了自己特殊的爱。"
},
{
"id" : "121",
"message_id" : 21,
"guild_id" : 139260719811133440,
"channel_id" : 139260947033358336,
"user_id" : 139259716026105856,
"timestamp" : "2020-09-02T14:14:44.000Z",
"content" : "In a very real sense, each one of us, as humans beings, have been given a gold container filled with unconditional love and kisses... from our children, family members, friends, and God. There is simply no other possession, anyone could hold, more precious than this."
},
{
"id" : "122",
"message_id" : 22,
"guild_id" : 139260719811133440,
"channel_id" : 139260947033358336,
"user_id" : 139259448643420160,
"timestamp" : "2020-09-02T14:14:44.000Z",
"content" : "从一个非常真实的意义上说，我们每个人都被赠与过一个无形的金色礼盒，那里面装满了来自子女，家人，朋友及上帝无条件的爱与吻。人们所能拥有的最珍贵的礼物莫过于此了。"
},
{
"id" : "123",
"message_id" : 23,
"guild_id" : 139260719811133440,
"channel_id" : 139260947033358336,
"user_id" : 139259619326427136,
"timestamp" : "2020-09-02T14:14:44.000Z",
"content" : "上面这两篇短文都是有中英文对照的，在选择优秀的英文美文的时候我们也要注意中英文对比。在读的时候尽量不要看中文的翻译意思，要按照自己的能力进行分析，感受。在读完之后可以，对照中文再进行阅读，这样会让大家的英语能力有所提升。"
}
]
"#;

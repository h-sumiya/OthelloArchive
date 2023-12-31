use crate::base::Board; //python:del
use crate::pos::Pos; //python:del
use std::collections::HashMap;
use std::mem::transmute; //python:del

const BOOK:[u8;5182] = include_data!(5182, "嬥䌒䴚䄢㌊娐䐙儂䠄㰋㠞怩搲挬崻䘅伕嘼񃼮嬥䌒䴚崬刔優搳䄩戽氘㴻倕㬐㈄䤅堌䘰񃼃嬥䴢娔崓挬䨞䈦㬵㴴䔌䘺伅弧欮㼗䠙񃼑嬥䴢娔崓挬䨞䈦㬵㴴䔌䘺伅弧欮㼗䠙񃼑嬥䴢娔崓挬䨞䈦㬵㴴䔌䘺伅弧欮㼗䠙񃼊嬥䌒䴚䔭搬帪攽刋䤔氠挻戩堖伞模䀹嗿䈫䨓尝䐭娢戩搡挕䄙㰍㔃㈊䀄倘帵嗿别䐝䌪尭丳嘚攒搋㰍樕㔖土帯񃼄嬥䴢娔崓挬䨞䈦㬵㴴䔌㐺欲䘼񃼏嬥䌒䴚䄢䤊尔崪䠠堡戳水欩攖񃼦嬥䌒䴚䄢䤊尔崪䠠堡戳水洩攖񃼦嬥䌒䴚䔔䤬崪㰢㬅㨍㌑搐㄂䘞񃼦嬥䌒䴚䐪刋崬㈙㨦䄮世䠗伕㴧񃼅嬥䴢娔崓挬䨞䈦㬵㴴䔺欲䘼伌񃼅嬥䌒䴚䔭搬刵帩㰔娦弻䜞洳堋嗿䈫䨓初㨑娙尋倡䀨䠩䐈丕㴂񃼖嬥䴢娔崓挬䨞䈦㬵㴴䔺欲䘼񃼏嬥䌒䴚崬刔優搳䄩戽樘攼䔐񃼞嬥䌒䴚䔔娬㬞帢㰭嘖攍欴㐅񃼯嬥䌒䴚䔭搬帪攽刯挩並䤖儲嗿䈫䨓初儙䔪䐠䠊堑挩怲㬬񃼐嬥䌒䴚嘭攬刳䐖搪樋戍夕儞嗿䈫䨓崝丕䐦尗帖圵㼟弢㰴񃼋嬥䌒䴚崬刔太儲挴標欵䔖㰠嗿别䐝䌪尭丳嘚攒搋㰍樕㔖񃼽嬥䴢娔嘓崬䨖䄙䠡䀈㰠㌋񃼂嬥䴢娔崓挬䨖䄙䈦倡䀘堩񃼻嬥䴢娔崓挬䨞䈦㬵㰂䘡引񃼟嬥䴢娔崓挬戴䔚樻㰹氩攽񃼞嬥䌒䴚䔭䤔尦帖㰡㴋㔄㈢嗿䈫䨓崝尕攴夢䐮嘌氪挞񃼽嬥䌒䴚䔭搬刵帩娦儼㰔㨖嗿䈫䨓崝尦䐢䘕丳搮攽気񃼻嬥䔚嘝䌭䐬搒刞帯伍㬌䘩嗿䈫䨓尝㨑㬔则挪儩㴮㰕񃼠嬥䌒䄚䴪刬帡夕搨崲䤔攳嗿䨫娦崢尝䘵搳串圽帓䈔񃼕嬥䴢娔崓挬䨞䈦䄙倡䀘戩嗿别䐝䌪尭搳䨲欕攺椽㴌񃼡嬥䴢娔䔓崬䘞圦䜟弮㰍㌅嗿䈫䨓崝尕挴洮嘵伯䜞朧嗿䈫䨓崝尕挴洮嘵伯䜞朧嗿䈫䨓崝尕挴洮嘵伯䜞朧嗿䈫䨓尝㰑䤢娋䐠䔡㈞䘩嗿䈫䨓䐝尕中嘗伖㰪帍㼢嗿䈫䨓䔝尔㨭㰢攙挦搮夞嗿䈫䨓初㨑䐙娬倭儘挨񃼻嬥䌒䴚䄢䤊尔个戭㰳㬠嗿䈫崝䌕䐚丌㬬䤦帗㨴񃼡嬥䴒嘭搬挢䔖䨔䌪㴋㰞嗿别䐝尪中嘚挒戵䌴帼񃼺嬥䴢娔攬戳搭帕䨻洼嘾嗿䈫䨓崝尕攴夢嘮氪㬡嗿䈫䨓崝尕攴夢嘮氪㬡嗿䨫刓䐝㨕尪下儩䤒㤂嗿䨫刓䐝㨕尪下儩䤒㤂嗿䨫挑䐝娬刭夡搲樼帕嗿䈫䨓崝尕挴欮娢樲渵嗿䈫䨓初儙䔪䐬崮挦攖嗿䈫䨓䐝娭挢尴䘩帕儦嗿䨫刓挙䐝儕崬䄨䈪丩嗿䨫初嘭尓娮攴欳民両嗿䈫刓䔝䨔嘮尪㴭㐌弟嗿别䐝䌪尭丳帚搦攽圯嗿别䐝䌪尭搳䨲欕为椗嗿䈫䨓崝尕挴洮嘵伯񃼼嬥䌒䴚䔭搬帳攽弦氟嗿䈫䨓初㨑娬儙䐋㈨񃼳嬥䌒䴚䄢尊䤪㬡堔挂嗿䈫䨓初㨑娬儙䐋㈨񃼩嬥䌒䴚䄢尊䤪㬡堔夂嗿尭䐝娢搫䈳䘲攌樓񃼑嬥䄚䴙嘭䌕䐒刬䀊㬌嗿䨫䤑崦䔝䈓尔㨢㰐񃼋嬥䌒䴚䔭搬帳刦欩截嗿䈫䨓初䄙挊尪倡䀨񃼩嬥䌚䴢䔔帞圦弗伖崷嗿䨫娦崢尝䘵临䈓儕񃼳嬥䴢娔崓挬䨞䈴洦攮嗿别䐝尪挭搲欼攺䘽񃼚嬥䴢娔嘕崬丮䌳䈴䤚嗿别䐝䌪崦帕䘒丬㨵񃼚嬥䌒䴚䄢尊䤪㬡堔񃼲嬥䌒䴚䄢尊䤪㬡堔񃼕嬥嘒䌭䔔䴮搬㰵丢񃼪嬥䌒䴚嘭䔔䜞刬䤖񃼡嬥䔒嘝刪䌚挴戬洭񃼯嬥䌒䴚崬刔太儲挴񃼵嬥䌒䴚崬刔太儲搳񃼵嬥䌒䴚䄬刌䐪崋䤡񃼊嬥嘚刪䴭攬搖䔞弧񃼲嬥䴢娔崓挬戴洚儩񃼨嬥䴢娔崬䨞䈦帵䌴񃼲嬥䌒䴚䄢䐊娋䠙夡嗿䈫䨓初㨑䀭夡㴔񃼉嬥䴢娔崓挬䨖䈦㰙嗿尭䐝娢搫䈵䘲氽񃼍崥䴬刔嬪攴戒洖㴼嗿尭嘝丮攴弽土欷񃼫崥䴬带搞洵伯朧嬻嗿尭嘝丮攴伯朧欽񃼫崥䴬嬒䨕䤢䀠優夨嗿䈫䨓崝尕挴嘮氵񃼢嬥䌒䴚䄢㌊娐䐙䔡嗿䈫䨓初㨑䤬䐕丌񃼋嬥䌒䴚䤢娑㈊䐬䔋嗿䈫䨓初䤡㨑䀔䠪񃼕嬥䌒䴚䄬䐌刕䀙儍嗿䈫䨓娑挩㨼刬䀙񃼝嬥䴢䐓嘞䨬娖㨕崒嗿别䌝䈚尪䐡丕㴋񃼵嬥䴢娔嘓䔭圞尮䘯嗿䈫䨓崝尕挴嘮娢嗿䈫䨓初㨑㬔䤪䘐嗿䈫䨓初㨑㬔䤪㴡嗿䈫䨓崝尕娴嘮䘵嗿䈫䨓崝尕娴嘮䘵嗿䈫䨓崝尕挴嘮娢嗿䈫䨓崝尕挴洮張嗿䈫䨓崝尕挴洮張嗿䈫䨓崝尕挴洮㰵嗿䈫䨓崝尕挴洮㰵嗿䈫䨓初㨑㬔䤪丐嗿䈫䨓初㨑㬔䤪丐嗿䨫刓䐝㰕尪挭携嗿尭帝丫氵渽圯朗嗿䈫䨓崝尕挴嘮娢嗿䈫䨓崝尦䐳䔢头嗿䈫䨓崝尦䐳䔢圴嗿䨫初嘭䔯䐓娮㬒嗿䈫娓尝䤢䨡崩倔嗿䨫娦尢䈔䤲崳䔝嗿别䐝䌪尭䘳帚񃼒嬥䌒䴚䔭搬刵䤩嗿䈫䨓崝尕攴夢񃼙嬥䌒䴚䔭搬䄳㰔嗿䈫䨓崝尕挴洮񃼢嬥䌒䴚䔭搬帳刽嗿䈫䨓崝䐦丕䘗񃼍嬥䌒䴚䄢䤊㰔㌠嗿䈫䨓初㨑娬儙񃼭嬥䌒䴚䄢䐊娋崙嗿䈫䨓初㨑㬔䤪񃼬嬥䌚䴬䔔崪带弴嗿䐝䌭䨒尕嬪帢񃼯嬥䄚尪䌲䴳崢搦嗿䈫䨓初䄙娬䐡񃼍嬥䌒䴚䄢娋尡崙嗿䨫䈓初㨑㬔䤪񃼡嬥䌚䴒䄢䐊娋䠙嗿䨫䈓初㨑㬔䤪񃼐嬥䌒䴚嘭挬帔刖嗿䈫䨓崝䐦丕䘗񃼍嬥䌚䴬䔔䤪嘞䄒嗿䨫刓䐝㨕个尋񃼖嬥䴪嘬䨓刔䤑儒嗿别䌝且尦㰭䘗񃼕嬥䴢䌔嘬䈚崋儊嗿䈫䨓尝䐭娢挩嗿䈫䨓崝尕挴攮嗿䈫䨓崝尕挴攮嗿䈫䨓初㨑娬候嗿䈫崝䐚㴕娬䤢嗿䈫崝䐚㴕娬䤢嗿别䐝䌪尭䘳搚嗿别䐝䌪尭䘳搚嗿别䐝䌪尭丳䤚嗿别䌝崔娬䄚䔒嗿尭䐝䨢临挖攟嗿尭䐝䨢娴攌䔳嗿尭䐝䨢娴攌䔳嗿尭䐝娢搫䈵串嗿尭䐝娢搫䈵挲嗿尭䐝帢嘕䌖㔌嗿尭䐝帢嘕䌖㔌嗿尭嘝攴䐢䨕㨓嗿尭嘝攴䐢䨕㨓嗿䨫我娓䈝䔊挬嗿䈫䨓崝尦攳夢嗿䈫䨓尝䐭娢挩嗿䨫初嘭䔯㴔㰓嗿䨫初尓䐡䔪㬵嗿䨫娦崢尝䘵帕嗿䨫娦尢儒䤓䐭嗿䐝㰍䌭䔚㔄񃼆嬥䌒䴚䤭尕嘔嗿䈫䨓初㨑娬񃼩嬥䌒䴚䄢䐊夋嗿䈫䨓初㨑㬔񃼩嬥䌒䴚䄢䐊䤋嗿䈫䨓初㨑㬔񃼄嬥嘚刪䌳䈙䔔嗿别䐝尪挵䔼񃼭嬥䴢娔䔓攬䈞嗿别䐝䌪尕丵񃼒嬥䴢娔崓挬䘞嗿尭丝攴䐦圓񃼪崥䴬搞嘵䌔娧嗿䐝䌭䨒嬢尕񃼪嬥䌒䴚䤭尕㴔嗿䨫娦夢䐒挡񃼓嬥嘚刪䌳䈙䔔嗿䈫崝䔓我搳񃼻嬥䌚䴢䈔㰋崙嗿䨫娦尢䤲挭񃼺嬥嘚刪戬䴙崴嗿别䌝且尦㰚񃼭嬥䴢娔㰓䈕㬚嗿别䐝䌪丌㬕񃼮嬥䌒䴚嘭刬񃼕嬥䌒䴚䔭䘞񃼢嬥䌒䴚嘭䔔񃼮嬥䌒䴚嘭䔔񃼬嬥䌒䴚嘭挬񃼲嬥䌒䴚嘭挬񃼴嬥䌒䴚䄢尊񃼡嬥䴒䌭䔔對񃼳嬥䌒䴢䨔娕񃼲嬥䌚䴢䔔䤊񃼑嬥䔲䨒䌔䴢񃼖嬥䔲䨒䴔䌢񃼖崥䴬刔䘦䔞񃼚崥䴬刔䘦䔞񃼮崥䴬搦䔔䈓񃼎崥䴬搦䔔䈓񃼎崥䴬搦帔䨫񃼵崥䴬搦帔䨫񃼵崥䴬搦帔䨫񃼵崥䴬刮嘞䐕񃼚崥䴬娮嬞刚񃼲崥䴬娶嬞刚񃼲嬥䌒䴚䄢䐌񃼊嬥嘒刪多䴭񃼕嬥䌒䴚嘭刬񃼕嬥䌒䴚嘭挬񃼲嬥䌒䴚嘭攬񃼮嬥䌚䴢䔔䤊񃼑嬥䴢娔攬崳񃼡崥䴬刔搚㰡嗿尭䐝䨢䔓񃼌崥䴬刔挚㰪嗿尭嘝攴氧񃼢嬥䌒䴚䔭嘔嗿䈫䨓崝䐕񃼦嬥䌒䴚䤭㰊嗿䈫䨓崝㨙񃼕嬥䌒䴚䤢儑嗿䈫䨓初㨑񃼋嬥䌒䴚䄢㬊嗿䨫尦刓娡񃼳嬥嘚䌬儢挪嗿别䐝䌪尭񃼵嬥䴢娔崓䈬嗿尭䐝䨢娴񃼋崥䴬刔搚㬪嗿尭䐝䨢儴񃼌崥䴬刔䌚㰕嗿尭䐝丳帕񃼌崥䴬搦圵刼嗿尭嘝䐴刕񃼧崥䴬搦䔔䌢嗿尭嘝䐴刕񃼯嬥䌒䴚䄢娌嗿䨫娦尓崳񃼲嬥嘚䌮䴢䔔嗿䨫刓䐝㨕񃼭嬥䌚䴢崋娳嗿䨫娦尢䤲񃼳嬥嘚刪䈬䴡嗿䨫娦尢䌔񃼒嬥䴪帬崦刞嗿䈫崝䔓刑嗿䈫䨓䐝丕嗿䈫䨓尝㬑嗿䈫䨓崝娦嗿䈫䨓崝娦嗿䈫䨓崝䔦嗿䈫崝㴕娬嗿别䌝且㴦嗿别搝嘭䐬嗿尭䐝䨢䈓嗿尭䐝䨫挌嗿尭䐝嘢師嗿尭嘝䔫戮嗿尭嘝攴䔢嗿尭帝丢伦嗿䈫䨓初㤑嗿别䐝䔭䌚嗿别䐝䔭䌚嗿䈫崝娓񃼮崥䴬搦嬵嗿䈫崝䔓񃼞嬥䄚嘙䌪嗿别䐝尪񃼙崥䴬刔㬚嗿尭䐝䨢񃼫崥䴬挔䜞嗿尭䐝丳񃼗崥䴬搦嬵嗿尭嘝䐴񃼞䴥崔㰓㴕嗿尭嘝挴嗿尭䈝䔳嗿尭帝䔢嗿䈫䨓挝嗿䈫䨓挝嗿䨫刓挝嗿䨫刓㰝嗿别䐝尚嗿别䐝挪嗿尭䐝搫嗿尭嘝挴嗿尭帝䔢嗿尭䈝䔳嗿尭䔝攢嗿尭䔝攢嗿尭䘝丕嗿䨫刓㰝嗿䨫刓搝嗿䨫刓挝嗿䈫䨓񃼡崥䴬䨒嗿尭䘝񃼫嬥䌒帢嗿䈫刓񃼮嬥䴢帔嗿别䐝񃼦崥䴬䌔嗿尭嘝񃼪崥䴬攮嗿尭䈝񃼚崥䴬嬖嗿䐝䨭񃼌嬥䴢񃼳嬥嘚񃼭崥䴬񃼓嬥嘚񃼭嬥䴢񃼕嬥䴢񃼳崥䴬񃼓崥嬬񃼲䴥嬔񃼕嬥䴒񃼬嬥䌚񃼪嬥䌚񃼲嬥䤢嗿别񃼙嬥夢嗿䌭񃼝崥䴚嗿䐝񃼋䴥񃼓䴥񃼕");
const MIRROR: [u8; 64] = include_data!(
    64,
    "㠀䠐堠栰㤁䤑夡椱㨂䨒娢樲㬃䬓嬣欳㰄䰔尤水㴅䴕崥洵㸆世带渶㼇众弧漷"
);
const ROTATE: [u8; 64] = include_data!(
    64,
    "㼇众弧漷㸆世带渶㴅䴕崥洵㰄䰔尤水㬃䬓嬣欳㨂䨒娢樲㤁䤑夡椱㠀䠐堠栰"
);

pub static mut KILLER: HashMap<(u64, u64), Pos> = unsafe { transmute([1u8; 48]) };

fn mirror(v: u8) -> u8 {
    if v < 64 {
        MIRROR[v as usize]
    } else {
        v
    }
}

fn rotate(v: u8) -> u8 {
    if v < 64 {
        ROTATE[v as usize]
    } else {
        v
    }
}
pub fn load_book() {
    let mut book = Vec::with_capacity(BOOK.len() * 4);
    book.extend_from_slice(&BOOK);
    for b in BOOK.iter() {
        book.push(mirror(*b));
    }
    for b in BOOK.iter() {
        book.push(rotate(rotate(*b)));
    }
    for b in BOOK.iter() {
        book.push(mirror(rotate(rotate(*b))));
    }
    unsafe {
        let mut data = HashMap::new();
        let mut board = Board::new();
        for d in book.iter() {
            if d == &255 {
                board = Board::new();
                continue;
            }
            let p = Pos(1 << d);
            data.insert((board.me, board.opp), p);
            board = board.put(p);
        }
        std::mem::swap(&mut data, &mut KILLER);
        std::mem::forget(data);
        eprintln!("book loaded {} moves", KILLER.len()); //python:debug
    }
}

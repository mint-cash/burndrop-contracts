use phf::phf_map;

// Compensation for the fee miscalculation
// address => oppamint amount (= ancs amount)
pub const COMPENSATION: phf::Map<&str, u128> = phf_map! {
    "terra10slxdut2mn70tl8wlj4hxarc3wglu2fwl3c3l4" => 200597785u128,
    "terra1229lq0aud2zx8hqchsqhanu97xynm64djy882j" => 400105098u128,
    "terra12zlat5jc0zsgd2t43sjc9h6kphmprqlhzefqkr" => 400109214u128,
    "terra152gzyueky6sl95w563jd3ft92vayywk89ql7ys" => 400126000u128,
    "terra16apa0qqjdzmutlwf6t7yyawkzkqk0kwa5eke68" => 400126000u128,
    "terra16j2vwm20a9vy4qqnjzfseg35hwvj7tuj3mndxa" => 400105046u128,
    "terra16yuytr0qvh7uq9lc02swsy3txzjp6heefas92q" => 800629454u128,
    "terra17suh8v8j4wpfspymnjylf87tn3lrcmaua9v2m6" => 400108114u128,
    "terra18054g235t975gtynw85h07lw44h5ee77pwwnax" => 400103930u128,
    "terra189jce3ch0l0w6uak5kezshrwhg9lp9rr8n2q9q" => 400103904u128,
    "terra18huxuv6rqhrkw25gnf4qphkp2sdqj97j8vsyv7" => 400105100u128,
    "terra18xmps02xqulvduh5kmsaug22w2f52qztslsdvt" => 400111470u128,
    "terra19vnazx7v6mqp24f6cqn8atz9hel9e7g9faj4ru" => 400109214u128,
    "terra19y2830kkwmsyretez2w0x4mzq0yet2m2ncp9fm" => 584375505u128,
    "terra1an8gk4zadh889us2x3he5cmr0ysm3mnfqv93ww" => 400103906u128,
    "terra1asly25wcccuat2jcn57tkzug4vxykkergcpst5" => 400105016u128,
    "terra1azes5x0mxglxjlap9lzft5umptrkd5zfan8g7v" => 5132126000u128,
    "terra1c2hx4m4nnks84530n557n8z7wcept8ne7gussy" => 400109255u128,
    "terra1ca0jctwnxz287xv93p9ng7dvt4gz803p2qxq54" => 400103786u128,
    "terra1cp85fgv2uhxvv9wrd7u98szwv9cs94cwxr8e4q" => 400105020u128,
    "terra1cywcngqq69jgn22lpgq3epmp2jvj4wh8zr4nm3" => 397726000u128,
    "terra1d3tkt76wt5yvge9kkazzt33jgs84jtrpvefys0" => 400126000u128,
    "terra1d4v6dcdj00v2l7vl2j4nee2prz7x5r3c3q68u9" => 400110413u128,
    "terra1ed3gvuseyxsxa60jjy647c535yfewcrmp5lw25" => 400090577u128,
    "terra1euqghzv5vra2rdu2krmj8rj58xl7yd9g7eam0u" => 1541716u128,
    "terra1f6n7fwy6kuzfmpv2dqmve8vlnkqvhnppv9z3es" => 400109032u128,
    "terra1ggh7w5v9t7ndf34cc0q3ty86lxamezf4qvljaa" => 400103843u128,
    "terra1gmtm6g4066jt8s52nvtr927f85c7wtd6jjdkmq" => 400109269u128,
    "terra1h0w67ysrzstmrwyg4uckdzl88phf0r7qewrtj2" => 4400126000u128,
    "terra1jh6hk5fh5gngr3dxcc9apkke0m8xsqn7xvgjv3" => 400109258u128,
    "terra1jmm7sru08xpww64m8v0g33zjdgkvqg8gd76j2d" => 400105084u128,
    "terra1jvsxwu92xl9mgkvme9rsmmk3xan2kzmh8ywl3j" => 12903674u128,
    "terra1kggx3dhq7t9cnumqhksqekx37ds0zx0keruvmj" => 508726u128,
    "terra1kqeer6wqcllga725cex4z53rp6g37x80ucug8q" => 400110371u128,
    "terra1lqp8wxcrdhxzzy8fs4w6whaysp82j055asg92w" => 400104938u128,
    "terra1lskrc49xg4k8x7sn58v34vh9nsa2rzqqqkh8pv" => 1200197968u128,
    "terra1mwk4y34kzvck3ptvaxtk8c8h2ahcg30gv84j9f" => 800232034u128,
    "terra1nptgynq230h8l5hdlwz62zz2c20xp385ndrgt9" => 400103931u128,
    "terra1nx2vmfs8fk3lwgsar7a93pwfuzk37r34smm8mq" => 440704482u128,
    "terra1nzc34h0e9we9v2mpv0z5auy9ttw79uwd3nsv33" => 440126000u128,
    "terra1p5m77hwtt96nyxq4asn8sthyhsw39x9mpssl77" => 437816458u128,
    "terra1psya92m4dhnawf78yju9g07evqs6fncrdvxzyj" => 480211780u128,
    "terra1q6n90yre90fczqekgww8ldr6ph57tul5t8huw7" => 440126000u128,
    "terra1qjtwuve8s62s0pn5y9afs8wjm2r59dduxlgxfk" => 400106270u128,
    "terra1ql2nlgw4h25lgln6uwxhff5t9t3jssqz22zc7d" => 1200378000u128,
    "terra1r3h9zwgk57yuavwpe064zcc5cjx8qw67pqgut0" => 400109186u128,
    "terra1r7m7j2gapkzvhnl08yuze6v29krd8j2znhtsxd" => 16952216612u128,
    "terra1rfz5zw3nu2qkfv34q7lq50yskeu6ulpgwyfj5u" => 15552926000u128,
    "terra1rrj9m9264mrxl2klc52kc58ax8lern3s59lj9h" => 400103860u128,
    "terra1rurcs4p6g9g536avcl5rug0gy9l267s4lymwz9" => 1200198376u128,
    "terra1rw43xy5388meu2pl3v02ckt8c754r73yhhv6qq" => 365093610u128,
    "terra1sejrtvwqgvpttr6rgw9d2jmh83fha8enmxls7x" => 5560126000u128,
    "terra1tvgwp7375cr6zm6pmny25qvv5w9ckarc9zkc04" => 400110330u128,
    "terra1u2m9vhfznxrpun2nf6q7xhlcmfcru7lq8nwvtg" => 400106270u128,
    "terra1utth2aenz3cvkgumrwgg8dmz94egw4p23d46y0" => 400109172u128,
    "terra1v8r2sw3tlysfey06ls82r43ycqg9qqzch568ac" => 400106187u128,
    "terra1vmwh0tskwuu2rm7t3gelus5dl3lsyszrkqrx92" => 2800197080u128,
    "terra1vrymnhgph5z8u7g29wymaw0c8eykg0yynptvpl" => 400109087u128,
    "terra1w0rnquxkud8e08ej0vtav3p7guey26rhch36ck" => 440126000u128,
    "terra1wm9mwmjgjt8wg2cwpya9c9pk5an9fwqet73tdj" => 400103846u128,
    "terra1wyhlxg3qk2dxw776a4pvyr77yslxj2p7lx4n97" => 596926000u128,
    "terra1x7kdtj5ljdr7p7ejdgs4vnsyu629yj5ssjf9jd" => 398106114u128,
    "terra1xkmuvjrf6z0exchnz4q70gakacjvsvx6vpnnlq" => 400090512u128,
    "terra1xp7x02vd353w5fveu52xaa3t70jkxkuyxdjd95" => 4220629524u128,
    "terra1xqnuhv9skn9se3ykl828l7w6lw09akhqmgzhd6" => 2526000u128,
    "terra1y0uyzzmlqf46kamp6lel9da3qlh3n87424mm0n" => 480181280u128,
    "terra1yx6p7y9f7gsp520p3nvqz23x09gtuv64m507r6" => 10520252000u128,
    "terra1yxwlw98hzue932q8c2klvtyjfzrc77m3wm68ds" => 400110286u128,
    "terra1zafpjz7h4x9cd8n5rf9lxz7tru352taxtl8jeu" => 400106150u128,
    "terra1zd0xdzr6p8xkse0cz0u2s26r235rl69je5nn0x" => 8000126000u128,
    "terra1zk8xnpfsjapa9f7qpqawx0nqa5tajzjk7wdn9r" => 800300448u128,
    "terra1zuk5h0mhqnhwez9qv9znl0hknmzmy0nd6et9aw" => 480212372u128,
    "terra1zy0skxw42gcdl490j648ulunc0aycvduzkk038" => 400105112u128,
    "terra1zycd89hudxkmjcgxtxahhudwhamg2wpvynkpdk" => 400106268u128,
};

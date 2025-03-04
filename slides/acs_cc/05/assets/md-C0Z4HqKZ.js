import{_ as p}from"./slidev/CodeBlockWrapper.vue_vue_type_script_setup_true_lang-DZMkRcp_.js";import{b as r,o as u,w as l,g as s,e as c,B as n,m as d,v as m,x as f,C as e}from"./modules/vue-2OMAnX-t.js";import{I as _}from"./slidev/default-BKWbm29S.js";import{u as h,f as g}from"./slidev/context-CgbHkRnR.js";import"./modules/unplugin-icons-BcKRKgpt.js";import"./index-Bw7vCG7C.js";import"./modules/shiki-DO6X-yZY.js";const k={grid:"~ cols-2 gap5"},T={__name:"slides.md__slidev_10",setup(x){const{$clicksContext:t,$frontmatter:i}=h();return t.setup(),(v,a)=>{const o=p;return u(),r(_,m(f(e(g)(e(i),9))),{default:l(()=>[a[2]||(a[2]=s("h1",null,"Tasks can stop the executor",-1)),s("div",k,[a[1]||(a[1]=s("div",null,[s("ul",null,[s("li",null,[n("unless awaited, "),s("code",null,"async"),n(" functions are not executed")]),s("li",null,[n("tasks have to use "),s("code",null,".await"),n(" in loops, otherwise they block the scheduler")])])],-1)),c(o,d({},{ranges:["all","5-8","3-9"]}),{default:l(()=>a[0]||(a[0]=[s("pre",{class:"shiki shiki-themes vitesse-dark vitesse-light slidev-code",style:{"--shiki-dark":"#dbd7caee","--shiki-light":"#393a34","--shiki-dark-bg":"#121212","--shiki-light-bg":"#ffffff"}},[s("code",{class:"language-text"},[s("span",{class:"line"},[s("span",null,"#[embassy_executor::task]")]),n(`
`),s("span",{class:"line"},[s("span",null,"async fn led_blink(mut led:Output<'static, PIN_X>) {")]),n(`
`),s("span",{class:"line"},[s("span",null,"    loop {")]),n(`
`),s("span",{class:"line"},[s("span",null,"        led.toogle();")]),n(`
`),s("span",{class:"line"},[s("span",null,"        // this does not execute anything")]),n(`
`),s("span",{class:"line"},[s("span",null,"        Timer::after_secs(1);")]),n(`
`),s("span",{class:"line"},[s("span",null,"        // infinite loop without `.await`")]),n(`
`),s("span",{class:"line"},[s("span",null,"        // that never gives up the MCU")]),n(`
`),s("span",{class:"line"},[s("span",null,"    }")]),n(`
`),s("span",{class:"line"},[s("span",null,"}")]),n(`
`),s("span",{class:"line"},[s("span")]),n(`
`),s("span",{class:"line"},[s("span",null,"#[embassy_executor::main]")]),n(`
`),s("span",{class:"line"},[s("span",null,"async fn main(spawner: Spawner) {")]),n(`
`),s("span",{class:"line"},[s("span",null,"    // ..")]),n(`
`),s("span",{class:"line"},[s("span",null,"    loop {")]),n(`
`),s("span",{class:"line"},[s("span",null,"        button.wait_for_rising_edge().await;")]),n(`
`),s("span",{class:"line"},[s("span",null,'        info!("button pressed");')]),n(`
`),s("span",{class:"line"},[s("span",null,"    }")]),n(`
`),s("span",{class:"line"},[s("span",null,"}")])])],-1)])),_:1},16)])]),_:1},16)}}};export{T as default};

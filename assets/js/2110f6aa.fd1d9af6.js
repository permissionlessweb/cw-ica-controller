"use strict";(self.webpackChunkdocs=self.webpackChunkdocs||[]).push([[263],{6104:(e,t,n)=>{n.r(t),n.d(t,{assets:()=>o,contentTitle:()=>i,default:()=>h,frontMatter:()=>c,metadata:()=>l,toc:()=>a});var s=n(5893),r=n(1151);const c={title:"Events",sidebar_label:"Events",sidebar_position:5,slug:"/contract-api/events"},i="Events",l={id:"contract-api/events",title:"Events",description:"The cw-ica-controller contract emits events upon various channel and packet lifecycle events.",source:"@site/versioned_docs/version-v0.20.x/contract-api/05-events.mdx",sourceDirName:"contract-api",slug:"/contract-api/events",permalink:"/cw-ica-controller/v0.20/contract-api/events",draft:!1,unlisted:!1,editUrl:"https://github.com/srdtrk/cw-ica-controller/tree/main/docs/versioned_docs/version-v0.20.x/contract-api/05-events.mdx",tags:[],version:"v0.20.x",sidebarPosition:5,frontMatter:{title:"Events",sidebar_label:"Events",sidebar_position:5,slug:"/contract-api/events"},sidebar:"docsSidebar",previous:{title:"Callbacks",permalink:"/cw-ica-controller/v0.20/contract-api/callbacks"},next:{title:"Introduction",permalink:"/cw-ica-controller/v0.20/how-it-works/introduction"}},o={},a=[{value:"Attributes",id:"attributes",level:2}];function d(e){const t={code:"code",h1:"h1",h2:"h2",header:"header",p:"p",strong:"strong",table:"table",tbody:"tbody",td:"td",th:"th",thead:"thead",tr:"tr",...(0,r.a)(),...e.components};return(0,s.jsxs)(s.Fragment,{children:[(0,s.jsx)(t.header,{children:(0,s.jsx)(t.h1,{id:"events",children:"Events"})}),"\n",(0,s.jsxs)(t.p,{children:["The ",(0,s.jsx)(t.code,{children:"cw-ica-controller"})," contract emits events upon various channel and packet lifecycle events.\nI haven't really thought about what events should be emitted, so this is a work in progress and feel free to open\nan issue if you have any suggestions. Changes in events will not be considered breaking changes as they are not\nstored on the blockchain, thus, can be included in patch releases."]}),"\n",(0,s.jsx)(t.p,{children:"Core IBC already emits events for channel and packet lifecycle events, so we keep the events to a minimum."}),"\n",(0,s.jsx)(t.p,{children:"The only events emitted by the contract are when an acknowledgement packet is received, so that an external\nindexer can index the result of the packet execution."}),"\n",(0,s.jsx)(t.h2,{id:"attributes",children:"Attributes"}),"\n",(0,s.jsx)(t.p,{children:"Whether or not the result of the packet execution was successful, the following attributes are emitted:"}),"\n",(0,s.jsxs)(t.table,{children:[(0,s.jsx)(t.thead,{children:(0,s.jsxs)(t.tr,{children:[(0,s.jsx)(t.th,{style:{textAlign:"center"},children:(0,s.jsx)(t.strong,{children:"Attribute Key"})}),(0,s.jsx)(t.th,{style:{textAlign:"center"},children:(0,s.jsx)(t.strong,{children:"Attribute Value"})})]})}),(0,s.jsxs)(t.tbody,{children:[(0,s.jsxs)(t.tr,{children:[(0,s.jsx)(t.td,{style:{textAlign:"center"},children:(0,s.jsx)(t.code,{children:"packet_sequence"})}),(0,s.jsx)(t.td,{style:{textAlign:"center"},children:"String"})]}),(0,s.jsxs)(t.tr,{children:[(0,s.jsx)(t.td,{style:{textAlign:"center"},children:(0,s.jsx)(t.code,{children:"packet_src_port"})}),(0,s.jsx)(t.td,{style:{textAlign:"center"},children:"String"})]}),(0,s.jsxs)(t.tr,{children:[(0,s.jsx)(t.td,{style:{textAlign:"center"},children:(0,s.jsx)(t.code,{children:"packet_src_channel"})}),(0,s.jsx)(t.td,{style:{textAlign:"center"},children:"String"})]})]})]}),"\n",(0,s.jsx)(t.p,{children:"If the packet execution was successful, then the following attributes are also emitted:"}),"\n",(0,s.jsxs)(t.table,{children:[(0,s.jsx)(t.thead,{children:(0,s.jsxs)(t.tr,{children:[(0,s.jsx)(t.th,{style:{textAlign:"center"},children:(0,s.jsx)(t.strong,{children:"Attribute Key"})}),(0,s.jsx)(t.th,{style:{textAlign:"center"},children:(0,s.jsx)(t.strong,{children:"Attribute Value"})})]})}),(0,s.jsx)(t.tbody,{children:(0,s.jsxs)(t.tr,{children:[(0,s.jsx)(t.td,{style:{textAlign:"center"},children:(0,s.jsx)(t.code,{children:"packet_ack_base64"})}),(0,s.jsx)(t.td,{style:{textAlign:"center"},children:"Base64 (String)"})]})})]}),"\n",(0,s.jsx)(t.p,{children:"If the packet execution was unsuccessful, then the following attributes are also emitted:"}),"\n",(0,s.jsxs)(t.table,{children:[(0,s.jsx)(t.thead,{children:(0,s.jsxs)(t.tr,{children:[(0,s.jsx)(t.th,{style:{textAlign:"center"},children:(0,s.jsx)(t.strong,{children:"Attribute Key"})}),(0,s.jsx)(t.th,{style:{textAlign:"center"},children:(0,s.jsx)(t.strong,{children:"Attribute Value"})})]})}),(0,s.jsx)(t.tbody,{children:(0,s.jsxs)(t.tr,{children:[(0,s.jsx)(t.td,{style:{textAlign:"center"},children:(0,s.jsx)(t.code,{children:"error"})}),(0,s.jsx)(t.td,{style:{textAlign:"center"},children:"String"})]})})]})]})}function h(e={}){const{wrapper:t}={...(0,r.a)(),...e.components};return t?(0,s.jsx)(t,{...e,children:(0,s.jsx)(d,{...e})}):d(e)}},1151:(e,t,n)=>{n.d(t,{Z:()=>l,a:()=>i});var s=n(7294);const r={},c=s.createContext(r);function i(e){const t=s.useContext(c);return s.useMemo((function(){return"function"==typeof e?e(t):{...t,...e}}),[t,e])}function l(e){let t;return t=e.disableParentContext?"function"==typeof e.components?e.components(r):e.components||r:i(e.components),s.createElement(c.Provider,{value:t},e.children)}}}]);
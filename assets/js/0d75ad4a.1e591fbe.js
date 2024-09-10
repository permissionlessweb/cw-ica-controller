"use strict";(self.webpackChunkdocs=self.webpackChunkdocs||[]).push([[8708],{6453:(e,c,n)=>{n.r(c),n.d(c,{assets:()=>i,contentTitle:()=>r,default:()=>d,frontMatter:()=>s,metadata:()=>l,toc:()=>o});var a=n(5893),t=n(1151);const s={title:"Callbacks",sidebar_label:"Callbacks",sidebar_position:4,slug:"/contract-api/callbacks"},r="Callbacks",l={id:"contract-api/callbacks",title:"Callbacks",description:"The cw-ica-controller contract provides a callback mechanism upon various channel and packet lifecycle events.",source:"@site/versioned_docs/version-v0.4.x/contract-api/04-callbacks.mdx",sourceDirName:"contract-api",slug:"/contract-api/callbacks",permalink:"/cw-ica-controller/v0.4/contract-api/callbacks",draft:!1,unlisted:!1,editUrl:"https://github.com/srdtrk/cw-ica-controller/tree/main/docs/versioned_docs/version-v0.4.x/contract-api/04-callbacks.mdx",tags:[],version:"v0.4.x",sidebarPosition:4,frontMatter:{title:"Callbacks",sidebar_label:"Callbacks",sidebar_position:4,slug:"/contract-api/callbacks"},sidebar:"docsSidebar",previous:{title:"QueryMsg",permalink:"/cw-ica-controller/v0.4/contract-api/query-msg"},next:{title:"Events",permalink:"/cw-ica-controller/v0.4/contract-api/events"}},i={},o=[{value:"<code>ReceiveIcaCallback</code> enum variant",id:"receiveicacallback-enum-variant",level:2},{value:"IcaControllerCallbackMsg",id:"icacontrollercallbackmsg",level:2},{value:"OnChannelOpenAckCallback",id:"onchannelopenackcallback",level:3},{value:"OnAcknowledgementPacketCallback",id:"onacknowledgementpacketcallback",level:3},{value:"OnTimeoutPacketCallback",id:"ontimeoutpacketcallback",level:3}];function h(e){const c={a:"a",admonition:"admonition",code:"code",h1:"h1",h2:"h2",h3:"h3",header:"header",li:"li",ol:"ol",p:"p",pre:"pre",strong:"strong",ul:"ul",...(0,t.a)(),...e.components};return(0,a.jsxs)(a.Fragment,{children:[(0,a.jsx)(c.header,{children:(0,a.jsx)(c.h1,{id:"callbacks",children:"Callbacks"})}),"\n",(0,a.jsxs)(c.p,{children:["The ",(0,a.jsx)(c.code,{children:"cw-ica-controller"})," contract provides a callback mechanism upon various channel and packet lifecycle events.\nA callback address is recorded in the contract's state. This address is set during the contract's instantiation\nor can be updated later by the contract owner using ",(0,a.jsx)(c.a,{href:"/cw-ica-controller/v0.4/contract-api/execute-msg#updatecallbackaddress",children:(0,a.jsx)(c.code,{children:"ExecuteMsg::UpdateCallbackAddress"})}),"."]}),"\n",(0,a.jsxs)(c.h2,{id:"receiveicacallback-enum-variant",children:[(0,a.jsx)(c.code,{children:"ReceiveIcaCallback"})," enum variant"]}),"\n",(0,a.jsxs)(c.p,{children:["The contract whose address is recorded as the callback address must include a callback enum variant in its ",(0,a.jsx)(c.code,{children:"ExecuteMsg"})," enum.\nWe included a procedural macro to generate this enum variant for you in ",(0,a.jsx)(c.code,{children:"cw-ica-controller"}),"'s ",(0,a.jsx)(c.code,{children:"helpers"})," module. See the following example:"]}),"\n",(0,a.jsx)(c.pre,{children:(0,a.jsx)(c.code,{className:"language-rust",metastring:'title="src/msg.rs"',children:"use cw_ica_controller::helpers::ica_callback_execute;\n\n#[ica_callback_execute]\n#[cw_serde]\npub enum ExecuteMsg {}\n"})}),"\n",(0,a.jsx)(c.p,{children:"This will transform the enum to:"}),"\n",(0,a.jsx)(c.pre,{children:(0,a.jsx)(c.code,{className:"language-rust",metastring:'title="src/msg.rs"',children:"#[cw_serde]\npub enum ExecuteMsg {\n    ReceiveIcaCallback(::cw_ica_controller::types::callbacks::IcaControllerCallbackMsg),\n}\n"})}),"\n",(0,a.jsxs)(c.admonition,{type:"note",children:[(0,a.jsxs)(c.p,{children:["Other derive macro invocations must occur after this procedural macro as they may depend on the new variant. For example, the following will ",(0,a.jsx)(c.strong,{children:"fail"})," because the ",(0,a.jsx)(c.code,{children:"Clone"})," derivation occurs before the addition of the field."]}),(0,a.jsx)(c.pre,{children:(0,a.jsx)(c.code,{className:"language-rust",children:"use cw_ica_controller::helper::ica_callback_execute;\nuse cosmwasm_schema::cw_serde;\n\n#[derive(Clone)]\n#[ica_callback_execute]\n#[allow(dead_code)]\n#[cw_serde]\nenum Test {\n    Foo,\n    Bar(u64),\n    Baz { foo: u64 },\n}\n"})})]}),"\n",(0,a.jsxs)(c.p,{children:["Since this variant is inserted to the ",(0,a.jsx)(c.code,{children:"ExecuteMsg"}),", the callback contract must handle this enum variant in its ",(0,a.jsx)(c.code,{children:"execute"})," function:"]}),"\n",(0,a.jsx)(c.pre,{children:(0,a.jsx)(c.code,{className:"language-rust",metastring:"reference",children:"https://github.com/srdtrk/cw-ica-controller/blob/v0.4.1/testing/contracts/callback-counter/src/contract.rs#L28-L40\n"})}),"\n",(0,a.jsx)(c.p,{children:"The callback contract can then handle the callback message as it sees fit, ideally by performing some kind of validation that the callback comes\nfrom an expected legitimate source. The callback contract can also perform some kind of action based on the callback message, such as\nincrementing a counter or error handling."}),"\n",(0,a.jsxs)(c.admonition,{type:"warning",children:[(0,a.jsxs)(c.p,{children:["If the callback contract returns an error, the ",(0,a.jsx)(c.code,{children:"cw-ica-controller"})," will not proceed with the channel or packet lifecycle event that triggered the callback."]}),(0,a.jsxs)(c.ol,{children:["\n",(0,a.jsxs)(c.li,{children:["If the callback contract returns an error in response to a ",(0,a.jsx)(c.code,{children:"ChannelOpenAck"})," callback, then the ",(0,a.jsx)(c.code,{children:"cw-ica-controller"})," will not proceed with the channel opening."]}),"\n",(0,a.jsxs)(c.li,{children:["If the callback contract returns an error in response to a ",(0,a.jsx)(c.code,{children:"OnAcknowledgementPacketCallback"})," callback, then the ",(0,a.jsx)(c.code,{children:"cw-ica-controller"})," will not proceed\nwith the packet acknowledgement."]}),"\n",(0,a.jsxs)(c.li,{children:["If the callback contract returns an error in response to a ",(0,a.jsx)(c.code,{children:"OnTimeoutPacketCallback"})," callback, then the ",(0,a.jsx)(c.code,{children:"cw-ica-controller"})," will not proceed with the packet timeout."]}),"\n"]}),(0,a.jsx)(c.p,{children:"Since ICA channels are ordered, cases 2 and 3 will result in the halting of the channel until the callback contract returns a successful response."})]}),"\n",(0,a.jsx)(c.h2,{id:"icacontrollercallbackmsg",children:"IcaControllerCallbackMsg"}),"\n",(0,a.jsxs)(c.p,{children:["The ",(0,a.jsx)(c.code,{children:"IcaControllerCallbackMsg"})," enum is the message type that is sent to the callback contract. It contains the following variants:"]}),"\n",(0,a.jsx)(c.pre,{children:(0,a.jsx)(c.code,{className:"language-rust",metastring:"reference",children:"https://github.com/srdtrk/cw-ica-controller/blob/v0.4.1/src/types/callbacks.rs#L15-L46\n"})}),"\n",(0,a.jsx)(c.h3,{id:"onchannelopenackcallback",children:"OnChannelOpenAckCallback"}),"\n",(0,a.jsxs)(c.p,{children:["The ",(0,a.jsx)(c.code,{children:"OnChannelOpenAckCallback"})," variant is sent to the callback contract when the ",(0,a.jsx)(c.code,{children:"cw-ica-controller"})," receives a ",(0,a.jsx)(c.code,{children:"ChannelOpenAck"})," message from the counterparty chain."]}),"\n",(0,a.jsx)(c.p,{children:"Let's go through the fields of this variant:"}),"\n",(0,a.jsxs)(c.ul,{children:["\n",(0,a.jsxs)(c.li,{children:["\n",(0,a.jsxs)(c.p,{children:[(0,a.jsx)(c.strong,{children:(0,a.jsx)(c.code,{children:"channel"})}),": This is the IBC Channel that would be opened if the callback contract returns a successful response.\nSee ",(0,a.jsx)(c.a,{href:"https://github.com/CosmWasm/cosmwasm/blob/v1.5.2/packages/std/src/ibc.rs#L115-L128",children:(0,a.jsx)(c.code,{children:"cosmwasm_std::IbcChannel"})})," for more details."]}),"\n"]}),"\n",(0,a.jsxs)(c.li,{children:["\n",(0,a.jsxs)(c.p,{children:[(0,a.jsx)(c.strong,{children:(0,a.jsx)(c.code,{children:"ica_address"})}),": This is the address (in the counterparty chain) of the interchain account that would be created if the callback contract returns a successful response."]}),"\n"]}),"\n",(0,a.jsxs)(c.li,{children:["\n",(0,a.jsxs)(c.p,{children:[(0,a.jsx)(c.strong,{children:(0,a.jsx)(c.code,{children:"tx_encoding"})}),": This is the transaction encoding that would be used for the ICS-27 transactions."]}),"\n"]}),"\n"]}),"\n",(0,a.jsx)(c.h3,{id:"onacknowledgementpacketcallback",children:"OnAcknowledgementPacketCallback"}),"\n",(0,a.jsxs)(c.p,{children:["The ",(0,a.jsx)(c.code,{children:"OnAcknowledgementPacketCallback"})," variant is sent to the callback contract when the ",(0,a.jsx)(c.code,{children:"cw-ica-controller"})," receives an acknowledgement packet from the counterparty chain for a packet that was sent from the calling ",(0,a.jsx)(c.code,{children:"cw-ica-controller"})," contract."]}),"\n",(0,a.jsx)(c.p,{children:"Let's go through the fields of this variant:"}),"\n",(0,a.jsxs)(c.ul,{children:["\n",(0,a.jsxs)(c.li,{children:[(0,a.jsx)(c.strong,{children:(0,a.jsx)(c.code,{children:"ica_acknowledgement"})}),": This is the acknowledgement data that was sent by the counterparty chain."]}),"\n"]}),"\n",(0,a.jsx)(c.pre,{children:(0,a.jsx)(c.code,{className:"language-rust",metastring:"reference",children:"https://github.com/srdtrk/cw-ica-controller/blob/v0.4.1/src/ibc/types/packet.rs#L169-L177\n"})}),"\n",(0,a.jsxs)(c.p,{children:["If the ICA packet was successfully executed on the counterparty chain, then this will be ",(0,a.jsx)(c.code,{children:"Data::Result(Binary)"})," where ",(0,a.jsx)(c.code,{children:"Binary"})," is the protobuf encoded result of the ICA packet execution."]}),"\n",(0,a.jsxs)(c.p,{children:["If the ICA packet was not successfully executed on the counterparty chain, then this will be ",(0,a.jsx)(c.code,{children:"Data::Error(String)"})," where ",(0,a.jsx)(c.code,{children:"String"})," is the error message returned by the counterparty chain."]}),"\n",(0,a.jsx)(c.admonition,{type:"note",children:(0,a.jsxs)(c.p,{children:["The error message returned by the counterparty chain doesn't include any useful information about the error. It only\ncontains the codespace of the error. This is because in CosmosSDK, tx error messages may be non-deterministic, and\nthus, they cannot be included in the IBC packet acknowledgement which is a deterministic message. This is a limitation of ",(0,a.jsx)(c.code,{children:"ibc-go"}),"."]})}),"\n",(0,a.jsxs)(c.ul,{children:["\n",(0,a.jsxs)(c.li,{children:["\n",(0,a.jsxs)(c.p,{children:[(0,a.jsx)(c.strong,{children:(0,a.jsx)(c.code,{children:"original_packet"})}),": This is the original ICA packet that was sent by the calling ",(0,a.jsx)(c.code,{children:"cw-ica-controller"})," contract. See ",(0,a.jsx)(c.a,{href:"https://github.com/CosmWasm/cosmwasm/blob/v1.5.2/packages/std/src/ibc.rs#L195-L207",children:(0,a.jsx)(c.code,{children:"cosmwasm_std::IbcPacket"})})]}),"\n"]}),"\n",(0,a.jsxs)(c.li,{children:["\n",(0,a.jsxs)(c.p,{children:[(0,a.jsx)(c.strong,{children:(0,a.jsx)(c.code,{children:"relayer"})}),": This is the address of the relayer that relayed the packet to the counterparty chain."]}),"\n"]}),"\n"]}),"\n",(0,a.jsx)(c.h3,{id:"ontimeoutpacketcallback",children:"OnTimeoutPacketCallback"}),"\n",(0,a.jsxs)(c.p,{children:["The ",(0,a.jsx)(c.code,{children:"OnTimeoutPacketCallback"})," variant is sent to the callback contract when the ",(0,a.jsx)(c.code,{children:"cw-ica-controller"})," receives a timeout packet for a packet that was sent."]}),"\n",(0,a.jsx)(c.p,{children:"Let's go through the fields of this variant:"}),"\n",(0,a.jsxs)(c.ul,{children:["\n",(0,a.jsxs)(c.li,{children:["\n",(0,a.jsxs)(c.p,{children:[(0,a.jsx)(c.strong,{children:(0,a.jsx)(c.code,{children:"original_packet"})}),": This is the original ICA packet that was sent by the calling ",(0,a.jsx)(c.code,{children:"cw-ica-controller"})," contract. See ",(0,a.jsx)(c.a,{href:"https://github.com/CosmWasm/cosmwasm/blob/v1.5.2/packages/std/src/ibc.rs#L195-L207",children:(0,a.jsx)(c.code,{children:"cosmwasm_std::IbcPacket"})})]}),"\n"]}),"\n",(0,a.jsxs)(c.li,{children:["\n",(0,a.jsxs)(c.p,{children:[(0,a.jsx)(c.strong,{children:(0,a.jsx)(c.code,{children:"relayer"})}),": This is the address of the relayer that relayed the packet to the counterparty chain."]}),"\n"]}),"\n"]})]})}function d(e={}){const{wrapper:c}={...(0,t.a)(),...e.components};return c?(0,a.jsx)(c,{...e,children:(0,a.jsx)(h,{...e})}):h(e)}},1151:(e,c,n)=>{n.d(c,{Z:()=>l,a:()=>r});var a=n(7294);const t={},s=a.createContext(t);function r(e){const c=a.useContext(s);return a.useMemo((function(){return"function"==typeof e?e(c):{...c,...e}}),[c,e])}function l(e){let c;return c=e.disableParentContext?"function"==typeof e.components?e.components(t):e.components||t:r(e.components),a.createElement(s.Provider,{value:c},e.children)}}}]);
"use strict";(self.webpackChunkdocs=self.webpackChunkdocs||[]).push([[3976],{2053:(e,s,n)=>{n.r(s),n.d(s,{assets:()=>o,contentTitle:()=>i,default:()=>h,frontMatter:()=>l,metadata:()=>a,toc:()=>c});const a=JSON.parse('{"id":"intro","title":"Introduction","description":"release-plz-logo","source":"@site/docs/intro.md","sourceDirName":".","slug":"/","permalink":"/docs/","draft":false,"unlisted":false,"editUrl":"https://github.com/MarcoIeni/release-plz/tree/main/website/docs/intro.md","tags":[],"version":"current","frontMatter":{"slug":"/"},"sidebar":"tutorialSidebar","next":{"title":"CLI Usage","permalink":"/docs/usage/"}}');var r=n(4848),t=n(8453);const l={slug:"/"},i="Introduction",o={},c=[{value:"No more manual releases",id:"no-more-manual-releases",level:2},{value:"Release-plz features",id:"release-plz-features",level:2},{value:"Releases made easy",id:"releases-made-easy",level:2}];function d(e){const s={a:"a",code:"code",h1:"h1",h2:"h2",header:"header",img:"img",li:"li",ol:"ol",p:"p",ul:"ul",...(0,t.R)(),...e.components};return(0,r.jsxs)(r.Fragment,{children:[(0,r.jsx)(s.header,{children:(0,r.jsx)(s.h1,{id:"introduction",children:"Introduction"})}),"\n",(0,r.jsx)(s.p,{children:(0,r.jsx)(s.img,{alt:"release-plz-logo",src:n(4057).A+"",width:"973",height:"922"})}),"\n",(0,r.jsx)(s.h2,{id:"no-more-manual-releases",children:"No more manual releases"}),"\n",(0,r.jsx)(s.p,{children:"Releasing Rust packages is tedious and error-prone, just like most IT manual tasks.\nFor every package you want to release, you need to:"}),"\n",(0,r.jsxs)(s.ul,{children:["\n",(0,r.jsxs)(s.li,{children:["Increase the version in ",(0,r.jsx)(s.code,{children:"Cargo.toml"}),"."]}),"\n",(0,r.jsx)(s.li,{children:"Update the changelog."}),"\n",(0,r.jsxs)(s.li,{children:["Publish the package in the cargo registry (for example, ",(0,r.jsx)(s.a,{href:"https://crates.io",children:"crates.io"}),")."]}),"\n",(0,r.jsx)(s.li,{children:"Create and push a git tag."}),"\n"]}),"\n",(0,r.jsxs)(s.p,{children:["Meet ",(0,r.jsx)(s.a,{href:"https://github.com/MarcoIeni/release-plz",children:"release-plz"}),", a Rust open-source\nproject that automates these tasks, allowing developers to release Rust packages\nwithout the command line."]}),"\n",(0,r.jsx)(s.p,{children:"Release-plz creates a fully automated release pipeline, allowing you to\neasily release changes more frequently, without the fear of\ndoing typos or other\nsubtle manual mistakes you can make when releasing from your terminal."}),"\n",(0,r.jsx)(s.p,{children:"Here's an example of a release Pull Request created on the release-plz repository itself:"}),"\n",(0,r.jsx)(s.p,{children:(0,r.jsx)(s.img,{alt:"pr",src:n(9084).A+"",width:"2298",height:"1466"})}),"\n",(0,r.jsx)(s.h2,{id:"release-plz-features",children:"Release-plz features"}),"\n",(0,r.jsxs)(s.ul,{children:["\n",(0,r.jsxs)(s.li,{children:["Version update based on ",(0,r.jsx)(s.a,{href:"https://www.conventionalcommits.org/",children:"conventional commits"}),"."]}),"\n",(0,r.jsxs)(s.li,{children:["Changelog update with ",(0,r.jsx)(s.a,{href:"https://git-cliff.org",children:"git-cliff"}),",\nusing the ",(0,r.jsx)(s.a,{href:"https://keepachangelog.com/en/1.1.0/",children:"keep a changelog"})," format by default."]}),"\n",(0,r.jsxs)(s.li,{children:["API breaking changes detection with ",(0,r.jsx)(s.a,{href:"https://github.com/obi1kenobi/cargo-semver-checks",children:"cargo-semver-checks"}),"."]}),"\n",(0,r.jsx)(s.li,{children:"Cargo workspaces support."}),"\n",(0,r.jsx)(s.li,{children:"No configuration required."}),"\n",(0,r.jsxs)(s.li,{children:["Optional ",(0,r.jsx)(s.code,{children:"cargo update"})," before releasing."]}),"\n",(0,r.jsx)(s.li,{children:"Git tag created for every released package."}),"\n",(0,r.jsx)(s.li,{children:"Package published to any cargo registry."}),"\n",(0,r.jsx)(s.li,{children:"GitHub/Gitea releases."}),"\n"]}),"\n",(0,r.jsx)(s.h2,{id:"releases-made-easy",children:"Releases made easy"}),"\n",(0,r.jsx)(s.p,{children:"Release-plz makes releasing Rust packages child's play:"}),"\n",(0,r.jsxs)(s.ol,{children:["\n",(0,r.jsx)(s.li,{children:"For every commit, release-plz creates a release Pull Request from CI."}),"\n",(0,r.jsx)(s.li,{children:"The release Pull Request reminds the maintainer about the unpublished changes."}),"\n",(0,r.jsx)(s.li,{children:"The maintainer reviews and merges the pull request."}),"\n",(0,r.jsx)(s.li,{children:"Release-plz releases the updated packages from CI."}),"\n"]}),"\n",(0,r.jsx)(s.p,{children:'In short, release-plz makes releasing Rust packages as easy as clicking the pull\nrequest "merge" button.'}),"\n",(0,r.jsx)(s.p,{children:"Additionally, publishing creates with release-plz from CI adds visibility and\ntransparency to the release process."})]})}function h(e={}){const{wrapper:s}={...(0,t.R)(),...e.components};return s?(0,r.jsx)(s,{...e,children:(0,r.jsx)(d,{...e})}):d(e)}},9084:(e,s,n)=>{n.d(s,{A:()=>a});const a=n.p+"assets/images/pr-83eb2c4059cd3991cd92b3b43e156baf.png"},4057:(e,s,n)=>{n.d(s,{A:()=>a});const a=n.p+"assets/images/robot_text-b5725e5eeb52cd79aefb1c81b24b1256.jpeg"},8453:(e,s,n)=>{n.d(s,{R:()=>l,x:()=>i});var a=n(6540);const r={},t=a.createContext(r);function l(e){const s=a.useContext(t);return a.useMemo((function(){return"function"==typeof e?e(s):{...s,...e}}),[s,e])}function i(e){let s;return s=e.disableParentContext?"function"==typeof e.components?e.components(r):e.components||r:l(e.components),a.createElement(t.Provider,{value:s},e.children)}}}]);
webpackJsonp([3],{Y36v:function(t,n,e){"use strict";function r(t,n){if(!t)throw new ReferenceError("this hasn't been initialised - super() hasn't been called");return!n||"object"!=typeof n&&"function"!=typeof n?t:n}function c(t,n){if("function"!=typeof n&&null!==n)throw new TypeError("Super expression must either be null or a function, not "+typeof n);t.prototype=Object.create(n&&n.prototype,{constructor:{value:t,enumerable:!1,writable:!0,configurable:!0}}),n&&(Object.setPrototypeOf?Object.setPrototypeOf(t,n):t.__proto__=n)}Object.defineProperty(n,"__esModule",{value:!0});var o=e("KM04"),u=(e.n(o),e("E/bI")),i=e("PbA0"),f=(e.n(i),e("/QC5")),a=e("wtIQ"),b=Object(o.h)(i.Cell,{width:12,middle:!0},Object(o.h)("h1",null,"Dashboard")),d=Object(o.h)(i.Cell,{width:12,middle:!0},Object(o.h)("h3",null,"Categories")),h=Object(o.h)("p",null,"Loading..."),l=function(t){function n(){return r(this,t.apply(this,arguments))}return c(n,t),n.prototype.componentWillMount=function(){this.props.getData()},n.prototype.render=function(t){var n=t.categories,e=t.threads,r=t.pending,c=t.mobile;return Object(o.h)(i.Grid,{style:{"max-width":"900px",margin:"0 auto",padding:"10px"}},b,d,r>0&&h,n.slice(0,6).map(function(t){return Object(o.h)(i.Cell,{width:12},Object(o.h)(i.Cell,{width:12,middle:!0},Object(o.h)("h4",null,t.title)),e.filter(function(n){return n.category_id===t.id}).map(function(t){return Object(o.h)(i.Cell,{left:c?1:4,width:c?12:6},Object(o.h)(i.Card,null,Object(o.h)(i.CardHeader,{title:t.title}),Object(o.h)(i.CardBody,null,t.description),Object(o.h)(i.CardFooter,{right:Object(o.h)(i.Button,{onClick:function(){return Object(f.b)("/thread/"+t.id)}},"See comments")})))}))}))},n}(o.Component),p=function(t){return{getData:function(){t(Object(a.b)()),t(Object(a.c)())}}},O=function(t){var n=t.content;return{pending:n.pending,categories:n.categories,threads:n.threads,mobile:t.media.mobile}};n.default=Object(u.b)(O,p)(l)},wtIQ:function(t,n,e){"use strict";e.d(n,"f",function(){return i}),e.d(n,"b",function(){return f}),e.d(n,"e",function(){return a}),e.d(n,"c",function(){return b}),e.d(n,"a",function(){return d}),e.d(n,"d",function(){return h});var r=e("vyGZ"),c=function(t){return Object(r.a)("/api"+t)},o=Object(r.c)("/api/content"),u=Object(r.b)("CONTENT_REQUEST_ERROR"),i=function(t){return function(n){n(Object(r.b)("GET_USER_PENDING")()),c("/user/"+t).then(function(t){return n(t)}).catch(function(t){return n(u(t))})}},f=function(){return function(t){t(Object(r.b)("GET_ALL_CATEGORIES_PENDING")()),c("/categories").then(function(n){return t(n)}).catch(function(n){return t(u(n))})}},a=function(t){return function(n){n(Object(r.b)("GET_THREAD_PENDING")()),c("/thread/"+t).then(function(t){return n(t)}).catch(function(t){return n(u(t))})}},b=function(){return function(t){t(Object(r.b)("GET_ALL_THREADS_PENDING")()),c("/threads").then(function(n){return t(n)}).catch(function(n){return t(u(n))})}},d=function(t,n,e,c){return function(i){i(Object(r.b)("ADD_COMMENT_PENDING")()),o(Object(r.b)("ADD_COMMENT")({threadId:t,userId:n,parentId:e,content:c})).then(function(t){return i(t)}).catch(function(t){return i(u(t))})}},h=function(t){return function(n){n(Object(r.b)("GET_COMMENTS_IN_THREAD_PENDING")({id:t})),c("/thread/"+t+"/comments").then(function(t){return n(t)}).catch(function(t){return n(u(t))})}}}});
//# sourceMappingURL=route-dashboard.chunk.e2e64.js.map
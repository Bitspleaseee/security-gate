webpackJsonp([2],{"45S/":function(t,n,e){"use strict";function r(t,n){if(!t)throw new ReferenceError("this hasn't been initialised - super() hasn't been called");return!n||"object"!=typeof n&&"function"!=typeof n?t:n}function c(t,n){if("function"!=typeof n&&null!==n)throw new TypeError("Super expression must either be null or a function, not "+typeof n);t.prototype=Object.create(n&&n.prototype,{constructor:{value:t,enumerable:!1,writable:!0,configurable:!0}}),n&&(Object.setPrototypeOf?Object.setPrototypeOf(t,n):t.__proto__=n)}Object.defineProperty(n,"__esModule",{value:!0});var o=e("KM04"),u=(e.n(o),e("E/bI")),i=e("PbA0"),a=(e.n(i),e("/QC5")),f=e("CSCC"),b=e("FEQE"),h=e("wtIQ"),l=Object.assign||function(t){for(var n=1;n<arguments.length;n++){var e=arguments[n];for(var r in e)Object.prototype.hasOwnProperty.call(e,r)&&(t[r]=e[r])}return t},d=function(t){function n(){for(var n,e,c,o=arguments.length,u=Array(o),i=0;i<o;i++)u[i]=arguments[i];return n=e=r(this,t.call.apply(t,[this].concat(u))),e.state={content:""},c=n,r(e,c)}return c(n,t),n.prototype.componentWillMount=function(){this.props.fetchData()},n.prototype.render=function(t,n){var e=t.isAuth,r=t.addComment,c=t.thread,u=t.comments,h=t.error,l=t.mobile,d=n.content;return Object(o.h)(i.Grid,{columns:l?5:12,style:{"max-width":"900px",margin:"0 auto",padding:"10px"}},h&&Object(o.h)(i.Cell,{left:l?1:4,width:5},Object(o.h)(b.a,{message:h,label:"Dashboard",onClick:function(){return Object(a.b)("/dashboard")}})),c&&Object(o.h)(i.Cell,{width:l?5:12,middle:!0},Object(o.h)("h1",null,c.title),Object(o.h)("p",null,c.description)),u.map(function(t){var n=t.content,e=t.user;return Object(o.h)(i.Cell,{left:l?1:4,width:5},Object(o.h)("p",null,n),Object(o.h)("p",null,Object(o.h)("small",null,e.username)))}),e&&Object(o.h)(i.Cell,{left:l?1:4,width:5},Object(o.h)("textarea",{rows:4,cols:50,value:d,onChange:Object(f.a)(this,"content")}),Object(o.h)(i.Button,{onClick:function(){return r({thread_id:c.id,content:d})}},"Add comment")))},n}(o.Component),s=function(t,n){var e=t.auth,r=t.content,c=t.media,o=parseInt(n.id),u={authUser:e.username,isAuth:e.authenticated,mobile:c.mobile};return isNaN(o)?l({},u,{error:"'"+n.id+"' is an invalid id (should be a number)",thread:null,comments:[]}):l({},u,{thread:r.threads.find(function(t){return t.id===o}),comments:r.comments.filter(function(t){return t.thread_id===o}).map(function(t){return l({},t,{user:n.getUser(t.user_id)})})})},O=function(t,n){return{fetchData:function(){t(Object(h.e)(n.id)),t(Object(h.d)(n.id))},getUser:function(n){return t(Object(h.f)(n))},addComment:function(n){return t(Object(h.a)(n))}}};n.default=Object(u.b)(s,O)(d)},wtIQ:function(t,n,e){"use strict";e.d(n,"f",function(){return i}),e.d(n,"b",function(){return a}),e.d(n,"e",function(){return f}),e.d(n,"c",function(){return b}),e.d(n,"a",function(){return h}),e.d(n,"d",function(){return l});var r=e("vyGZ"),c=function(t){return Object(r.a)("/api"+t)},o=Object(r.c)("/api/content"),u=Object(r.b)("CONTENT_REQUEST_ERROR"),i=function(t){return function(n){n(Object(r.b)("GET_USER_PENDING")()),c("/user/"+t).then(function(t){return n(t)}).catch(function(t){return n(u(t))})}},a=function(){return function(t){t(Object(r.b)("GET_ALL_CATEGORIES_PENDING")()),c("/categories").then(function(n){return t(n)}).catch(function(n){return t(u(n))})}},f=function(t){return function(n){n(Object(r.b)("GET_THREAD_PENDING")()),c("/thread/"+t).then(function(t){return n(t)}).catch(function(t){return n(u(t))})}},b=function(){return function(t){t(Object(r.b)("GET_ALL_THREADS_PENDING")()),c("/threads").then(function(n){return t(n)}).catch(function(n){return t(u(n))})}},h=function(t,n,e,c){return function(i){i(Object(r.b)("ADD_COMMENT_PENDING")()),o(Object(r.b)("ADD_COMMENT")({threadId:t,userId:n,parentId:e,content:c})).then(function(t){return i(t)}).catch(function(t){return i(u(t))})}},l=function(t){return function(n){n(Object(r.b)("GET_COMMENTS_IN_THREAD_PENDING")({id:t})),c("/thread/"+t+"/comments").then(function(t){return n(t)}).catch(function(t){return n(u(t))})}}}});
//# sourceMappingURL=route-thread.chunk.42134.js.map
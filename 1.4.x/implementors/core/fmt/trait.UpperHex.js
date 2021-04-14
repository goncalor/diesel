(function() {var implementors = {};
implementors["num_bigint"] = [{"text":"impl UpperHex for BigInt","synthetic":false,"types":[]},{"text":"impl UpperHex for BigUint","synthetic":false,"types":[]}];
implementors["tinyvec"] = [{"text":"impl&lt;A:&nbsp;Array&gt; UpperHex for ArrayVec&lt;A&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;A::Item: UpperHex,&nbsp;</span>","synthetic":false,"types":[]},{"text":"impl&lt;'s, T&gt; UpperHex for SliceVec&lt;'s, T&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;T: UpperHex,&nbsp;</span>","synthetic":false,"types":[]},{"text":"impl&lt;A:&nbsp;Array&gt; UpperHex for TinyVec&lt;A&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;A::Item: UpperHex,&nbsp;</span>","synthetic":false,"types":[]}];
implementors["uuid"] = [{"text":"impl UpperHex for Uuid","synthetic":false,"types":[]},{"text":"impl&lt;'a&gt; UpperHex for Simple&lt;'a&gt;","synthetic":false,"types":[]},{"text":"impl&lt;'a&gt; UpperHex for Hyphenated&lt;'a&gt;","synthetic":false,"types":[]}];
if (window.register_implementors) {window.register_implementors(implementors);} else {window.pending_implementors = implementors;}})()
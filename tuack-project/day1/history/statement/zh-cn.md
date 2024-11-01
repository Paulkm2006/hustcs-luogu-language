{{ self.title() }}

{{ s('description') }}

小 S 是历史长河。

一条历史长河是对一个文明从发展到衰亡的记录，若有好事者对这个文明感兴趣，便可以从历史长河中一探究竟。

历史长河是一段仅包含大小写字母的字符串，记录了每一个时间点这个文明发生的重要事件。字典序越小的字母代表的事件越重要。

现在，有两条不同的历史长河 $S$ 和 $T$，作为好事者的你正打算对比研究它们。在研究过程中，你遇到了 $Q$ 个问题，在每个问题中，你想要比较两个文明中特定时段的事件重要程度。

形式化的，每次给出 $l_s, r_s$ 和 $l_t, r_t$，你需要判断 $S[l_s, r_s]$ 和 $T[l_t, r_t]$ 谁的字典序更小。

其中， $S[l,r]$ 表示从字符串 $S$ 的第 $l$ 个字符到第 $r$ 个字符连起来构成的字符串。例如，若 $S$ 为 `kitsuki`，则 $s[3,5]$ 为 `tsu`。

{{ s('input format') }}

第一行是一个字符串 $S$。 

第二行是一个字符串 $T$。 

第三行是一个整数，表示询问次数 $Q$。 

接下来 $Q$ 行，每行四个整数 $l_s, r_s, l_t, r_t$，表示一次询问。

{{ s('output format') }}

对每次询问，输出一行一个字符串：

- 如果 $S[l_s, r_s]$ 的字典序更小，请输出 `yifusuyi`。
- 如果 $T[l_t, r_t]$ 的字典序更小，请输出 `erfusuer`。
- 如果两者的字典序一样大，请输出 `ovo`。

{{ s('sample', 1) }}

{{ self.sample_text() }}

{{ s('subtasks') }}

对 $100\%$ 的数据，$1 \leq |S|, |T|, Q \leq 10^3$，$1 \leq l_s \leq r_s \leq |S|$，$1 \leq l_t \leq r_t \leq |T|$。输入字符串仅含大小写英文字母。其中 $|S|$ 表示历史长河 $S$ 的长度，$|T|$ 表示历史长河 $T$ 的长度。

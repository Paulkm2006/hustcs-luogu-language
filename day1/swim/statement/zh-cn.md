{{ self.title() }}

{{ s('description') }}

小 W 大三了，她选了游泳课。

虽然游泳很累，但是小 W 还是坚持游泳，可是她很快难过的发现，自己的力气不够，游泳好累哦。已知小玉第一步能游 $2$ 米，可是随着越来越累，力气越来越小，她接下来的每一步都只能游出上一步距离的 $98\%$。现在小 W 想知道，如果要游到距离 $s$ 米的地方，她需要游多少步呢。

{{ s('input format') }}

输入一个实数 $s$（单位：米），表示要游的目标距离。

{{ s('output format') }}

输出一个整数，表示小 W 一共需要游多少步。

{{ s('sample', 1) }}

{{ self.sample_text() }}

{{ s('subtasks') }}

对于 $100\%$ 的测试数据，保证 $0 \le s \le 100$，$s$ 小数点后最多只有一位。
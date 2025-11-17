// vector 动态数组
pub fn fn01() {
    let one = "\
01: The quick brown
02: fox jumps over
03: the la1zy dog
04: The quick brown
05: fox jumps over
06: the la2zy dog
07: The quick brown
08: fox jumps over
09: the la3zy dog
10: The quick brown
11: fox jumps over
12: the la4zy dog";

    // 刺激，目标字符串
    let needle = "a2";

    // 目标行的下标
    let mut tags: Vec<usize> = vec![];

    // 上下文 Vec，二维可变数组，将会存放每个匹配目标行的行下标和这行的内容，和它前面几行和后面几行的行下标和行内容
    let mut ctx: Vec<Vec<(usize, String)>> = vec![];
    let ctx_lines = 2;

    for (i, line) in one.lines().enumerate() {
        if line.contains(needle) {
            tags.push(i);

            // 匹配到最多的情况就是，每一行都包含目标字符串
            // 首先初始化一个容量为 5 的 Vec，然后加到 ctx Vec 中
            // 总是为 Vec 设定容量，是最佳实践，可以避免超限后重新分配内存的开销
            let v = Vec::with_capacity(2 * ctx_lines + 1);
            ctx.push(v);
        }
    }

    if tags.is_empty() {
        return;
    }

    // 查 tags Vec 的容量、长度和内容
    println!("tags: cap {}, len {}, {:?}", tags.capacity(), tags.len(), tags);

    // 查 ctx Vec 的容量、长度和内容
    println!("ctx: cap {}, len {}, {:?}", ctx.capacity(), ctx.len(), ctx);

    for (i, line) in one.lines().enumerate() {
        for (j, tag) in tags.iter().enumerate() {
            // saturating 意为饱和
            // sub 意为减法
            // tag 是 &usize 类型，没有直接减 usize 类型的 ctx_lines，而是通过 saturating_sub
            // 函数减，是为了避免 tag 比 ctx_lines 而发生差为负的情况，为负将在取值时引发下标越界异常
            let lower_bound = tag.saturating_sub(ctx_lines);
            let upper_bound = tag + ctx_lines;

            if (i >= lower_bound) && (i <= upper_bound) {
                let line_as_string = String::from(line);
                let local_ctx = (i, line_as_string);
                ctx[j].push(local_ctx);
            }
        }
    }

    println!("tags: cap {}, len {}, {:?}", tags.capacity(), tags.len(), tags);
    println!("ctx: cap {}, len {}, {:?}", ctx.capacity(), ctx.len(), ctx);

    for local_ctx in ctx.iter() {
        // ref 是借用，& 是取址
        for &(i, ref line) in local_ctx.iter() {
            let line_num = i + 1;
            println!("{}: {}", line_num, line);
        }
    }
}

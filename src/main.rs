// Box<T>
fn main() {
    // Box<T 允许将一个值存储在堆中，然后将其智能指针保存在栈中

    /*
     * Box<T> 的作用与场景
     * 1. Box 的功能单一仅仅是将数据存储在堆中
     * 2. 当数据较大时，又不想在转移所有权时拷贝新数据。例如 [0,1000]
     * 3. 某一数据类型的大小无法在编译期确定，但又需要固定其大小类型。 例如 Box<str>
     * 4. 特征对象，只关心某类数据具有相同的trait，不关心其具体的类型。 例如 Box<dyn Animal>
     */

    // 将数据保存在堆中
    data_on_heap();

    // 避免栈中数据拷贝
    no_copy();

    // 固化动态大小的数据
    curing_dynamic_data();
}

fn data_on_heap() {
    let a = 3; // 数据保存在栈中
    let b = Box::new(3); // 数据保存在堆中 b是智能指针保存在栈中，b指向堆中对应的数据

    println!("a = {a}, b = {b}"); // 可以打印出b，因为b被隐式的解引用了

    // let c = a + b; // 但在表达式中Box不会隐式的自动解引用
    let c = a + *b; // 手动解引用后可访问到数据
    println!("c = {c}");

    /*
     * a的内存由编译器自动管理 当变量离开作用域时自动释放
     * b离开作用域时，其真实堆中数据将被Drop trait来实现释放
     */
}

fn no_copy() {
    // 存储在栈中的数转移所有权时，其本质是对栈中的数据进行了拷贝
    // 例如：
    let a = [1, 2, 3];
    let b = a;
    // 当将a赋值给b时，调用了 Copy trait 。
    println!("a = {:?}, b = {:?}", a, b); // b 和 a 可以同时存在 因此a的所有权并没有被转移

    // 但若数据量非常大时
    let c = [1; 1000];
    let d = c; // 将发生复制行为，非常消耗内存
    println!("c.len = {:?}, d.len = {:?}", c.len(), d.len());

    // 将大数据使用Box包裹。使其存储在堆中
    let e = Box::new([0; 1000]);
    let f = e; // e 的所有权移动至f。

    // e不可再访问
    // println!("{}", e.len());
    println!("f.len = {}", f.len());
}

fn curing_dynamic_data() {
    // todo
}

# Lyzh4IR 设计初稿

---
title: Lyzh4IR设计
author: Lyzh流云坠海（刘知杭）

---

ir分为两套，一套用于给人看和机器处理，另一套交给vm运行

## SIR

SIR应为严格的、符合定义的SExpr（见[词法定义文件](https://github.com/pigeon-project/corn/blob/master/src/corn_cob/corn.pest)）

### 表现形式

采用s表达式的方式
标签作为特殊的ir表达式，Symbol前加`:`
任意的覆盖信息熵操作后必须带`!`

### 代码块样例

```scheme
(defn (function_name :debug :o3)
    (push_local local_address)
    (call veriable_name))
    (:label label_name)
    (jump_to level_name)
    (ret)
```

##  VMIR

Cov4VM是典型的栈机结构。
在废案中我试图将Cov4VM定义为一个拥有无限个虚拟寄存器的理想机器。随后我发现栈机能够很自然的转换到寄存器机模型上，而寄存器机模型则并不能转为栈机模型。因此LLIR那套方案在这里行不通

鉴于Cov4VM和JVM的相似性：例如两者都是动态语言虚拟机、有类似的对象内存模型。因此，我基于JVM设计了一套VMIR

## ByteCode文件元信息

一个完整的字节码文件需要包含包括`文件头`、`类型信息`、`常量池`和`代码块`

### 文件头

一个完整的文件头需要包含`一个魔数`、`一个主版本号和一个子版本号`和`一个源文件的相对路径`

#### 魔数

魔数由一个值为`0x66ccff(暂定)`的四字节无符号数构成
起到校验文件类型的作用

#### 版本号

版本号作为字节码的版本号而不是VM的版本号

大多数时候，字节码应该是向前兼容的，字节码的主版本号变动因用于放弃兼容性的破坏性改动

#### 源文件相对路径

用于Debug和可能的热加载

### 类型信息

VM支持包括
- `unit`
- `byte`
- `bool`
- `char`
- `u32`、`i32`
- `u64`、`i64`
- `f32`、`f64`
- `ref`
- `any`
- `union`
- `array`、`string`
在内的基本类型

需要注意的是：
- `bool`长度`>=1byte(8bit) && <=4byte(32bit)`
- `char`长度恒为`4byte(32bit)`
- `ref<T>`用于确定类型的堆上数据类型
- `union`仅作为计算`sizeof`时起效，作用类似C++语言中的`union`
- 对于所有`sizeof<=4byte(32bit)`的元素而言，我们在 Cov4VM 的默认实现（也是目前为止的事实标准）中均会对齐为`4byte(32bit)`

允许用户自定义结构
```scheme
(struct struct_name
    [item_name TypeExpr])
```
对于元组结构来说，索引应名字改编为__item_{index}
例如
```scheme
(struct NewType
    [__item_0 (ref TypeName)])
```
对于可能存在的TaggedUnion来说，应生成为以下固定格式
```scheme
(struct __Option_Some
    [__item_0 any])
(struct Option
    [__tag u8]
    [__value (union __Option_Some unit)])
```

### 常量池

常量池分为字面量池和符号信息池

#### 字面量池

#### 符号和引用信息池

### 代码块

#### 全局方法池

## 字节码

- static
    - `(nop)`
    - `(int <int-number>)`
    - `(load-const  <constant-pool:index>)`
    - `(load-local  <locals:index>)`
    - `(load-item   <item:index>)`
    - `(store-local <locals:index>)`
    - `(store-item  <item:index>)`
    - `(throw)`
    - `(return-32)`
    - `(return-64)`
    - `(new <typeinfo-pool:index>)`
    - `(newarray <length>)`
    - `(pop)`
    - `(pop2)`
    - `(swap)`
    - `(jump)`
    - `(if-true-jump)`
    - `(if-false-jump)`
    - `(if-non-zero-jump)`
    - `(is-nil)`
    - `(is-not-nil)`
    - `(eq-32)`
    - `(eq-64)`
    - `(neq-32)`
    - `(neq-64)`
    - `(ge-u32)`
    - `(ge-i32)`
    - `(ge-u64)`
    - `(ge-i64)`
    - `(ge-f32)`
    - `(ge-f64)`
    - `(le-u32)`
    - `(le-i32)`
    - `(le-u64)`
    - `(le-i64)`
    - `(le-f32)`
    - `(le-f64)`
    - `(geq-u32)`
    - `(geq-i32)`
    - `(geq-u64)`
    - `(geq-i64)`
    - `(geq-f32)`
    - `(geq-f64)`
    - `(leq-u32)`
    - `(leq-i32)`
    - `(leq-u64)`
    - `(leq-i64)`
    - `(leq-f32)`
    - `(leq-f64)`
    - `(and)`
    - `(or)`
    - `(not)`
    - `(bit-and)`
    - `(bit-or)`
    - `(bit-not)`
    - `(to-u8)`
    - `(to-u32)`
    - `(to-i32)`
    - `(to-u64)`
    - `(to-i64)`
    - `(to-f32)`
    - `(to-f64)`
    - `(add-u8)`
    - `(add-u32)`
    - `(add-i32)`
    - `(add-u64)`
    - `(add-i64)`
    - `(add-f32)`
    - `(add-f64)`
    - `(sub-u8)`
    - `(sub-u32)`
    - `(sub-i32)`
    - `(sub-u64)`
    - `(sub-i64)`
    - `(sub-i64)`
    - `(sub-f32)`
    - `(sub-f64)`
    - `(mul-u8)`
    - `(mul-u32)`
    - `(mul-i32)`
    - `(mul-u64)`
    - `(mul-i64)`
    - `(mul-i64)`
    - `(mul-f32)`
    - `(mul-f64)`
    - `(div-u8)`
    - `(div-u32)`
    - `(div-i32)`
    - `(div-u64)`
    - `(div-i64)`
    - `(div-i64)`
    - `(mul-f32)`
    - `(mul-f64)`
    - `(mod-u8)`
    - `(mod-u32)`
    - `(mod-i32)`
    - `(mod-u64)`
    - `(mod-i64)`
    - `(mod-i64)`
- dynamic
    - `(load-local-dyn  <symbol-pool:index>)`
    - `(load-item-dyn   <symbol-pool:index>)`
    - `(store-local-dyn <symbol-pool:index>)`
    - `(store-item-dyn  <symbol-pool:index>)`
    - `(store-item-dyn  <symbol-pool:index>)`
    - `(typeof      <typeinfo-pool:index>)`
    - `(instanseof  <typeinfo-pool:index>)`
    - `(dyn-cast    <typeinfo-pool:index>)`


### 字节码编号：随具体实现而定（暂时）
@ stack_demo.s — ARM32 栈操作演示
@ 展示 push/pop、fp 帧指针、局部变量操作
.arch armv7-a
.syntax unified
.text
.global main

main:
    @ 保存 lr 和 fp，建立帧
    push    {fp, lr}
    add     fp, sp, #4

    @ 加载一些值到寄存器
    mov     r0, #10
    mov     r1, #20
    mov     r2, #30

    @ 压栈保存
    push    {r0, r1, r2}

    @ 做些运算
    add     r3, r0, r1      @ r3 = 30
    mul     r4, r3, r2      @ r4 = 900

    @ 恢复寄存器
    pop     {r0, r1, r2}

    @ 返回
    mov     r0, #0
    pop     {fp, pc}

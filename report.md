# Requirement

how you implemented the LAB, what difficulties you encountered, and your own innovation (if any)…

let students who haven’t done the project to be able to understand what are you doing and what you’ve done after reading your report.

# Implement

Lab1:先从教程中大致了解本次Lab需要用的一些知识，然后分析代码，分析出Lab1中有一些变量没有用，有一些函数没有实现（在要求中也谈到了要实现的函数）。然后分析用到的结构体和已经实现的函数功能，补上一些需要的，还没有实现的功能，完成所需完成的函数（util::{get_rotation,get_view_matrix,get_model_matrix,get_projection_matrix}），完成Task1和Task2.

Lab2：先对比了Lab1和Lab2大概有哪些差别，然后类似于光追，先实现AABB，然后把三角形先按输入的顺序依次渲染出来验证AABB及其他渲染函数实现正确，接着使用depth_buf使得三角形输出的时候能按顺序渲染，实现基本的结果图，最后学习MSAA和FXAA并分别实现并对比时间和效果。

# Difficulties

1. 不知道为什么，Lab1渲染出来的三角形一开始是上下颠倒了，但是把初始的旋转角从0改为180可以暂时的解决这个问题。因为这个三角形是左右对称的，所以我们不知道左右有没有颠倒，但是确实实装了旋转。
2. 第一次在代码中使用别人实现好的矩阵，对于用法不熟悉，在网上找官方文档但并不太懂。

# Innovation

在Lab1中我发现color变量没有用到，所以更改了get_color函数并且把颜色这个参数加到了draw_line里面，使得画出来的三角形就像前面参数设置的一样，每条边有不同的颜色，方便区分左边和右边。

# what am I doing and what I’ve done

实现了一个简单的光栅化渲染器，有简单的使物体沿着某个轴转动的功能和2种反锯齿的功能，目前渲染出来的图片是一个三维空间中的三角形。


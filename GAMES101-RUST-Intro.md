# GAMES101-RUST-Intro

> This is the lab for JOHN-2023-ProgrammingPractices
>
> Teaching Assistants: [Kr.Cen](https://github.com/Kr-Panghu/)、[Y.W.Zhong](https://github.com/Danny2003/)、[Z.D.Yang](https://github.com/yzddzy/)

[toc]

## INTRO :rocket:

- **日程**

  - **Week 1**：介绍项目与前期准备（Task 0）、熟悉 Rust 语法（Task 1）、学习完成教程 book 1（Task 2）
  - **Week 2**：学习完成教程 book 2（Task 3）和多线程优化内容（Task 4）
  - **Week 3~4**：GAMES101-RUST 项目
  - **Week 4**：结课展示 & code review

- **分数**：

  **Raytracer:** 55分基础分，5分Bonus

  **GAMES101-RUST**：45分基础分，5分Bonus

  **结课展示** & **Presentation**：5分Bonus

  - | 任务                            | 分数占比             |
    | ------------------------------- | -------------------- |
    | **Raytracer Task 0**            | 5%                   |
    | **Raytracer Task 1**            | 5%                   |
    | **Raytracer Task 2**            | 15%                  |
    | **Raytracer Task 3**            | 20%                  |
    | **Raytracer Task 4**            | 10%                  |
    | **Rest of Raytracer**           | $\leq$ 5% (As bonus) |
    | **GAMES101 LAB**                | 45%  + 5% bonus      |
    | **结课展示** & **Presentation** | 5% （As bonus）      |

---

## LAB1: Rotate a triangle :bus:

In this lab, you need to imitate a simplified version of rasterizer renderer based on CPU.

### TASK1: Implement a rotated matrix and a perspective projection matrix

Given 3 points in 3D $v_0(2.0, 0.0, -2.0)$, $v_1(0.0, 2.0, -2.0)$, $v_2(-2.0, 0.0, -2.0)$, you should convert these coordinates into screen coordinates, and visualize the corresponding triangle.

In our code framework, we've already offered `draw_triangle` function, so you just need to construct transformational matrix.

What you need to do?

* Use `model`, `view`, `projection` function to visualize the triangle (exactly 3 lines), where the argument of `model` is `rotation_angle`. After you've done this part, when running the code, press `a` and `d` makes the triangle rotate around z-axis.

Hints: (this may help)

* `get_model_matrix(rotation_angle: f64)`：逐个元素地构建模型变换矩阵并返回该矩阵。在此函数中，你只需要实现三维中绕 z 轴旋转的变换矩阵，而不用处理平移与缩放。
* `get_view_matrix(eye_pos: V3d)`：根据摄像机的角度旋转物体（理解为摄像机摆正，物体随之旋转）
* `get_projection_matrix(eye_fov: f64, aspect_ratio: f64, z_near: f64, z_far: f64)`：使用给定的参数逐个元素地构建透视投影矩阵并返回该矩阵。

按 `a/d` 键旋转的效果示意图（由于摄像机沿着 Z 轴方向，因此绕 Z 轴旋转物体在光栅器上呈现效果应该保持形状不变）

![](https://notes.sjtu.edu.cn/uploads/upload_db857cd2849c04280fa627d6619e8688.png)

### TASK2: Let the triangle rotate around arbitrary axis

In TASK1, we've made the triangle rotate around z-axis by a mvp matrix (i.e., projection * view * model). Now, you need to do something challenging. Namely, let the triangle rotate around any axis as you set it by yourself.

What you need to do?

* Use **Rodrigues’ Rotation Formula** to let the triangle rotate around arbitrary axis. 

Hint: (this may help)

* 你可能需要额外写一些函数来实现第二步（绕任意轴旋转）
* 比如，你可以在光栅器内添加一个额外的 `arbitrary_rotation` 矩阵，在绘制三角形时(即 `draw_triangle` 函数)将原来的mvp矩阵乘上`arbitrary_rotation`。你可以写一个 `get_rotation(axis: Vector3<f64>, angle: f64)` 函数，当按下 `r` 键时进行任意轴旋转（这个参数要通过 std::io 在初始化时从控制台输入），此时调用`get_rotation`函数设置 `arbitrary_rotation`，当没有按下 `r` 键时，`arbitrary_rotation` 初始化为原来的方向(即参数为z-axis, 0度)。

按 `r` 键绕某设置好的任意轴旋转的效果示意图(下图为绕(1,2,3)方向旋转10度)

![](https://notes.sjtu.edu.cn/uploads/upload_72e5f1a88527606af1dcb3caaaeeecba.png)

---

## LAB2: Rasterize triangles :bus:

In LAB1, we only draw the triangle by 3 lines. Now, we want to color the triangle, i.e., set the pixels with color, this is actually rasterize the triangles(三角形栅格化). You need to implement a function `rasterize_triangle` .

What you need to do?

* Rasterize the triangle by function `rasterize_triangle(&mut self, t: &Triangle)` inside `rasterize`.

Hint: (this may help)

* This function is usually implemented as: (判断点是否在三角形内 & z-buffer算法)
  1. Create the AABB
  2. Pass through all pixels in AABB (by index). Then, the screen spatial coordinates at the center of the pixel were used to check whether the center point is within the triangle. (you may need a `inside_triangle` function with a bool return value)
  3. If `inside_triangle` is true, then compare the interpolated depth value (插值深度) at its location to the corresponding value in the buffer (深度缓冲). If the current point is closer to the camera, reset pixel color and update depth buffer.

After you've done this part, you may get a picture like (We've set the triangles properly, if you want to get something more fancy, you can reset the triangles)

![](https://notes.sjtu.edu.cn/uploads/upload_154c896eb5efdb3a9c4f61b4198a7b74.png)


If you enlarge this image, you may find some **alias**(黑边，即锯齿). In this part, you need to do **antialiasing**.

What you need to do?

> 用 super-sampling 处理 Anti-aliasing : 你可能会注意到，当我们放大图像时，图像边缘会有锯齿感。我们可以用 super-sampling 来解决这个问题，即对每个像素进行 2 * 2 采样，并比较前后的结果 (这里并不需要考虑像素与像素间的样本复用)。需要注意的点有，对于像素内的每一个样本都需要维护它自己的深度值，即每一个像素都需要维护一个 sample list。最后，如果你实现正确的话，你得到的三角形不应该有不正常的黑边。

* 要求1：你需要用至少两种方法处理抗锯齿，且必须完成 MSAA 方法 (Multi-Sampling Anti-Aliasing, 是 SSAA 的优化)，对于第二种方法，你可以去网上自行查阅抗锯齿优化的方法，也可以自己设计一种采样方法进行优化。

* 要求2：你需要给出抗锯齿优化前后的图形渲染效果对比图。

  对于 MSAA 方法，你应该可以得到一个比较好的修复结果图，如下图所示。

  ![](https://notes.sjtu.edu.cn/uploads/upload_43064b19605af17e3bde506912ced7da.png)


* 要求3：你需要给出抗锯齿优化前后的单次渲染所需的时间对比，观察抗锯齿优化会对性能产生多少 negative effects。RUST 关于这方面的 API，你可以使用 `std::time::Instant::now()` 来获取当前时间。

* 要求4：如 tutorial 中所述，你需要在 report 中给出你对光栅器产生走样(锯齿)的本质原因的理解。


==**Any thoughtful insights can be taken into account when calculating the bonus. You can even do anything beyond what we require you to do. If you do so, please indicate it in the report to make sure your TA knows.**==

## DEADLINE :alarm_clock:

You need to finish this part before **2023.7.9 23:59**, and write a report. The report should include: how you implemented the LAB, what difficulties you encountered, and your own innovation (if any).

You need to make sure the codes you submit can run succesfully. (Submit to Github)

You need to let students who haven't done the project to be able to understand what are you doing and what you've done after reading your report.

## REFERENCE :book:

This lab is mainly generated from **Graphics And Mixed Environemnt Seminar, Lingqi Yan, UCSB**.

GAMES101现代计算机图形学入门：https://sites.cs.ucsb.edu/~lingqi/teaching/games101.html
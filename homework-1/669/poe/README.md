# Homework-1 作业
为 PoE Pallet 编写测试用例
包括：
创建存证的测试用例 撤销存证的测试用例 转移存证的测试用例

## 我的实现：

包括了如下测试用例：

1. 成功创建存证
2. 在已有存证时创建触发失败
3. 成功撤销存证
4. 存证不存在时撤销操作失败
5. 撤销非自身拥有的存证失败
6. 转移存证成功
7. 转移不存在的存证时操作失败
8. 转移非自身拥有的存证操作失败
from knapsack import Knapsack
import unittest


class TestKnapsack(unittest.TestCase):

    def setUp(self) -> None:
        '''
        测试之前的准备工作
        :return:
        '''
        pass

    def tearDown(self) -> None:
        '''
        测试之后的收尾
        如关闭数据库
        :return:
        '''
        pass

    def test_result(self):
        self.assertEqual(Knapsack().knapsack(), 1)

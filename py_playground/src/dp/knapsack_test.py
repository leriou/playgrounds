from knapsack import Knapsack
import unittest


class TestKnapsack(unittest.TestCase):

    def test_result(self):
        self.assertEqual(Knapsack().knapsack(), 1)

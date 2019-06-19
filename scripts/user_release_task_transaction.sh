echo "Test Case 1: Cow-User Release transaction task: (False, cow user cannot release transaction)"
curl http://localhost:6789/release_task_transaction\
 -H "Content-Type:application/json"\
 -d '{"mid":1, "t_type":"生活用品", "info":"用到一半的洗面奶", "loss":2, "address":"SYSU"}'
echo ""

echo "Test Case 2: Student-User Release transaction task: (True)"
curl http://localhost:6789/release_task_transaction\
 -H "Content-Type:application/json"\
 -d '{"mid":2, "t_type":"生活用品", "info":"用到一半的洗面奶", "loss":2, "address":"SYSU"}'
echo ""

echo "Test Case 3: Student-User Release transaction task Duplication: (False, Cannot rewrite task!)"
curl http://localhost:6789/release_task_transaction\
 -H "Content-Type:application/json"\
 -d '{"mid":2, "t_type":"生活用品", "info":"用到一半的洗面奶", "loss":2, "address":"SYSU"}'
echo ""

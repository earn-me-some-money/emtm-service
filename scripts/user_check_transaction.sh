echo "Test Case 1: (True):"
curl http://localhost:6789/task/transaction\
 -H "Content-Type:application/json"\
 -X GET\
 -d '{"task_mid":2, "userid":"wechat12306", "poster_id":2}'
echo ""

echo "Test Case 2: (False, target mission is not transaction):"
curl http://localhost:6789/task/transaction\
 -H "Content-Type:application/json"\
 -X GET\
 -d '{"task_mid":1, "userid":"wechat12306", "poster_id":1}'
echo ""


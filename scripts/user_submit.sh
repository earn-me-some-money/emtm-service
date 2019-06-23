echo "Test Case 1: (True):"
curl http://localhost:6789/task/submit\
 -H "Content-Type:application/json"\
 -d '{"task_mid":1, "userid":"wechat12306", "student_id":2}'
echo ""

echo "Test Case 2: (False, current user is not mission poster):"
curl http://localhost:6789/task/submit\
 -H "Content-Type:application/json"\
 -d '{"task_mid":2, "userid":"wechat12307", "student_id":2}'
echo ""

echo "Test Case 3: (False, target student is not mission participant):"
curl http://localhost:6789/task/submit\
 -H "Content-Type:application/json"\
 -d '{"task_mid":2, "userid":"wechat12306", "student_id":1}'
echo ""

echo "Test Case 4: (False, mission not exist):"
curl http://localhost:6789/task/submit\
 -H "Content-Type:application/json"\
 -d '{"task_mid":5, "userid":"wechat12306", "student_id":2}'
echo ""
